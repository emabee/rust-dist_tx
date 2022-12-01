use crate::{
    rm::ResourceManager, tm::Status, tm::TransactionManager, ReturnCode, RmError, XaError,
    XaTransactionId,
};
use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use log::{debug, trace};
use std::{
    collections::{hash_map::DefaultHasher, HashMap},
    hash::{Hash, Hasher},
    io::Cursor,
    u64,
};

/// The format id that is used for the `XaTransactionId`s of `SimpleTransactionManager`.
const FORMAT_ID: i32 = 99;

/// `SimpleTransactionManager`
///
/// * identifies itself with a `u64` (hash of its name)
/// * identifies the resource managers with a `u64` given during the registration
/// * enumerates its global transactions with a `u64` counter,
///   starting with 0, or, if higher values are found during rm-registration,
///   with the highest found number (plus 1)
///
/// uses `XaTransactionId` with
///
/// * `format_id` = 99
/// * `global_tid`: u64 counter, starting from 0
/// * `branch_qualifier`: `tm_id`: u64, `rm_id`: u64
///
/// A minimal implementation of the `TransactionManager` interface.
///
/// Is identified with an application-defined String, whose hash is used as
/// `tm_id`.
///
/// No support is provided for multi-threading á la XA.
///
#[derive(Debug)]
pub struct SimpleTransactionManager {
    name: String,
    id: u64,
    rms: HashMap<u64, Box<dyn ResourceManager>>,
    last_gtid: u64,
    current_gtid: Option<u64>,
    status: Status,
}
impl SimpleTransactionManager {
    /// Produces a new instance.
    #[must_use]
    pub fn new<S: AsRef<str>>(name: S) -> SimpleTransactionManager {
        trace!("new()");
        let name = name.as_ref().to_string();
        let mut s = DefaultHasher::new();
        name.hash(&mut s);
        SimpleTransactionManager {
            name,
            id: s.finish() & (u64::max_value() - 0b_1111_1111_u64),
            rms: HashMap::<u64, Box<dyn ResourceManager>>::new(),
            last_gtid: 0,
            current_gtid: None,
            status: Status::IDLE,
        }
    }

    /// Returns the global transaction id that is currently used
    /// by this `SimpleTransactionManager`.
    pub fn get_gtid(&mut self) -> Option<u64> {
        self.current_gtid
    }

    fn next_global_tid(&mut self) -> u64 {
        self.last_gtid += 1;
        self.last_gtid
    }

    fn get_current_gtid(&mut self) -> Result<u64, XaError> {
        match self.current_gtid {
            None => Err(XaError::Usage("No current transaction set")),
            Some(u) => Ok(u),
        }
    }

    fn validate_and_set_status(&mut self, required: Status, new: Status) -> Result<(), XaError> {
        if required.contains(self.status) {
            self.status = new;
            Ok(())
        } else {
            Err(XaError::UsageDetails(format!(
                "SimpleTransactionManager is in state {:?}, but state {required:?} is required",
                self.status,
            )))
        }
    }

    /// Reports the name of this instance.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    fn rm_action<F>(&mut self, action: F, global_tid: u64) -> Result<(), XaError>
    where
        F: Fn(&mut Box<dyn ResourceManager>, &XaTransactionId) -> Result<ReturnCode, RmError>,
    {
        let mut errors = Vec::<RmError>::new();
        for (rm_id, rm) in &mut self.rms {
            let xatid = new_xatid(global_tid, self.id, *rm_id);
            if let Err(e) = action(rm, &xatid) {
                errors.push(e);
            }
        }
        if errors.is_empty() {
            Ok(())
        } else {
            Err(XaError::RmErrors(errors))
        }
    }

    fn rm_start(&mut self, global_tid: u64) -> Result<(), XaError> {
        self.rm_action(|rm, xatid| ((**rm).start(xatid)), global_tid)
    }

    // fn rm_join(&mut self, global_tid: &u64) -> Result<(),XaError> {
    //     self.rm_action(|rm, xatid| ((**rm).start_by_joining(xatid)), global_tid)
    // }

    // fn rm_resume(&mut self, global_tid: &u64) -> Result<(),XaError> {
    //     self.rm_action(|rm, xatid| ((**rm).start_by_resuming(xatid)), global_tid)
    // }

    // fn rm_suspend(&mut self, global_tid: &u64) -> Result<(),XaError> {
    //     self.rm_action(|rm, xatid| ((**rm).end_suspend(xatid)), global_tid)
    // }

    fn rm_end_success(&mut self, global_tid: u64) -> Result<(), XaError> {
        self.rm_action(|rm, xatid| ((**rm).end_success(xatid)), global_tid)
    }

    fn rm_end_failure(&mut self, global_tid: u64) -> Result<(), XaError> {
        self.rm_action(|rm, xatid| ((**rm).end_failure(xatid)), global_tid)
    }

    fn rm_prepare(&mut self, global_tid: u64) -> Result<(), XaError> {
        self.rm_action(|rm, xatid| ((**rm).prepare(xatid)), global_tid)
    }

    fn rm_commit(&mut self, global_tid: u64) -> Result<(), XaError> {
        self.rm_action(|rm, xatid| ((**rm).commit(xatid)), global_tid)
    }

    fn rm_commit_one_phase(&mut self, global_tid: u64) -> Result<(), XaError> {
        self.rm_action(|rm, xatid| ((**rm).commit_one_phase(xatid)), global_tid)
    }

    fn rm_rollback(&mut self, global_tid: u64) -> Result<(), XaError> {
        self.rm_action(|rm, xatid| ((**rm).rollback(xatid)), global_tid)
    }

    // fn rm_forget(&mut self, global_tid: &u64) -> Result<(),XaError> {
    //     self.rm_action(|rm, xatid| ((**rm).forget(xatid)), global_tid)
    // }

    // fn rm_recover(&mut self) -> XaResult<Vec<XaTransactionId>> {
    //     panic!("not yet implemented")
    // }

    fn try_rollback_after(
        &mut self,
        current_gtid: u64,
        method: &'static str,
    ) -> Result<(), XaError> {
        self.status = Status::ROLLINGBACK;
        let result = self.rm_rollback(current_gtid);
        if let Err(ref e) = result {
            trace_error(e, current_gtid, "error in rm_rollback");
        }
        self.status = Status::IDLE;
        match result {
            Err(e) => {
                if let XaError::RmErrors(v) = e {
                    Err(XaError::Inconsistency(
                        format!("rm_rollback() failed after a failed {method}()"),
                        v,
                    ))
                } else {
                    Err(e)
                }
            }
            Ok(()) => Ok(()),
        }
    }

    // fn is_my_xid(&self, xid: &XaTransactionId) -> bool {
    //     if xid.get_format_id() != FORMAT_ID {
    //         return false;
    //     }

    //     let bq: Vec<u8> = xid.get_branch_qualifier();
    //     if bq.len() != 16 {
    //         return false;
    //     };
    //     let mut rdr = Cursor::new(bq);

    //     rdr.read_u64::<LittleEndian>().unwrap() == self.id
    // }

    fn is_my_xid_and_rm(&self, xid: &XaTransactionId, rm_id: u64) -> bool {
        if xid.get_format_id() != FORMAT_ID {
            return false;
        }

        let bq = xid.get_branch_qualifier();
        if bq.len() != 16 {
            return false;
        };
        let mut rdr = Cursor::new(bq);

        if rdr.read_u64::<LittleEndian>().unwrap() != self.id {
            return false;
        }
        rdr.read_u64::<LittleEndian>().unwrap() == rm_id
    }
}

fn trace_error(e: &XaError, gtid: u64, method_name: &'static str) {
    if let XaError::RmErrors(ref vec_rmerr) = *e {
        for rm in vec_rmerr {
            trace!("{}({}) failed due to {:?}", method_name, gtid, rm);
        }
    } else {
        trace!("error in {}: {}", method_name, e);
    }
}

#[allow(clippy::similar_names)]
fn new_xatid(global_tid: u64, tm_id: u64, rm_id: u64) -> XaTransactionId {
    let mut v_gt = Vec::<u8>::with_capacity(64);
    v_gt.write_u64::<LittleEndian>(global_tid).unwrap();

    let mut v_bq = Vec::<u8>::with_capacity(128);
    v_bq.write_u64::<LittleEndian>(tm_id).unwrap();
    v_bq.write_u64::<LittleEndian>(rm_id).unwrap();

    XaTransactionId::try_new(FORMAT_ID, v_gt, v_bq).unwrap()
}

impl TransactionManager for SimpleTransactionManager {
    fn register(
        &mut self,
        mut rm: Box<dyn ResourceManager>,
        rm_id: u64,
        cleanup: bool,
    ) -> Result<(), XaError> {
        trace!("register(rm_id = {})", rm_id);
        if self.rms.get(&rm_id).is_some() {
            let errmsg = "cannot register with given rm_id, which is already in use";
            debug!("{}", errmsg);
            return Err(XaError::Usage(errmsg));
        }

        if cleanup {
            trace!("register(rm_id = {}) -> starting cleanup", rm_id);
            for xid in &(*rm).recover().unwrap_or_default() {
                trace!("found xid {:?}", xid);
                if self.is_my_xid_and_rm(xid, rm_id) {
                    trace!("trying to forget {:?}", xid);
                    (*rm).forget(xid).unwrap_or(ReturnCode::Ok);
                }
            }
        }

        self.rms.insert(rm_id, rm);
        Ok(())
    }

    fn unregister(&mut self, rm_id: u64) -> Result<(), XaError> {
        self.rms.remove(&rm_id);
        Ok(())
    }

    // Creates a new Global Transaction and tells all rms to start working for a
    // respective branch.
    //
    // TM must be in status TmStatus::Idle.
    // If successful, sets status to TmStatus::Active.
    // If not,???
    fn start_transaction(&mut self) -> Result<(), XaError> {
        trace!("start_transaction()");
        self.validate_and_set_status(
            Status::IDLE | Status::COMMITTED | Status::ROLLEDBACK,
            Status::ACTIVATING,
        )?;

        let global_tid = self.next_global_tid();

        trace!("start_transaction() -> rm_start({})", global_tid);
        match self.rm_start(global_tid) {
            Ok(()) => {
                self.current_gtid = Some(global_tid);
                self.status = Status::ACTIVE;
                return Ok(());
            }
            Err(e) => {
                trace!(
                    "start_transaction() -> rm_start({}) failed with {:?}",
                    global_tid,
                    e
                );

                trace!("start_transaction() -> rm_end_failure({})", global_tid);
                if let Err(XaError::RmErrors(v)) = self.rm_end_failure(global_tid) {
                    trace!(
                        "start_transaction() -> rm_end_failure({}) failed with {:?}",
                        global_tid,
                        v
                    );
                }

                trace!("start_transaction() -> rm_rollback({})", global_tid);
                if let Err(XaError::RmErrors(v)) = self.rm_rollback(global_tid) {
                    trace!(
                        "start_transaction() -> rm_rollback({}) failed with {:?}",
                        global_tid,
                        v
                    );
                }
            }
        }

        trace!(
            "start_transaction() -> rm_start({}), second attempt after cleanup",
            global_tid
        );
        match self.rm_start(global_tid) {
            Ok(()) => {
                self.current_gtid = Some(global_tid);
                self.status = Status::ACTIVE;
                Ok(())
            }
            Err(e) => {
                trace!(
                    "start_transaction() -> rm_start({}), second attempt failed, too",
                    global_tid
                );
                self.status = Status::IDLE;
                Err(e)
            }
        }
    }

    // Internally, does commit_one_phase if only a single RM is involved, otherwise
    // does 2PC: (end_success(), preprare(), commit() on all participating RMs)
    // Completes the transaction, if it is in state `TmStatus::Active`.
    //
    // If successful, the transaction is set to state `TmStatus::Committed`,
    // otherwise to `TmStatus::Failed` or `TmStatus::RolledBack`.
    fn commit_transaction(&mut self) -> Result<(), XaError> {
        trace!("commit()");
        let current_gtid = self.get_current_gtid()?;
        self.validate_and_set_status(Status::ACTIVE, Status::COMMITTING)?;

        // shortcut, if possible
        if self.rms.len() < 2 {
            trace!("commit() -> rm_commit_one_phase()");
            self.rm_commit_one_phase(current_gtid)?;
        } else {
            // 1. end_success()
            trace!("commit() -> rm_end_success()");
            if let Err(e) = self.rm_end_success(current_gtid) {
                trace_error(&e, current_gtid, "rm_end_success");
                self.try_rollback_after(current_gtid, "rm_end_success")?;
            }

            // 2. prepare()
            trace!("commit() -> rm_prepare()");
            if let Err(e) = self.rm_prepare(current_gtid) {
                trace_error(&e, current_gtid, "rm_prepare");
                self.try_rollback_after(current_gtid, "rm_prepare")?;
            }

            // 3. commit()
            trace!("commit() -> rm_commit()");
            if let Err(e) = self.rm_commit(current_gtid) {
                trace_error(&e, current_gtid, "rm_commit");
                self.try_rollback_after(current_gtid, "rm_commit")?;
            }
        }
        self.status = Status::COMMITTED;

        Ok(())
    }

    fn rollback_transaction(&mut self) -> Result<(), XaError> {
        trace!("rollback()");
        let current_gtid = self.get_current_gtid()?;
        match self.status {
            Status::ACTIVE => {
                trace!("rollback() ACTIVE -> rm_end_failure()");
                self.rm_end_failure(current_gtid)?;
                self.rm_rollback(current_gtid)?;
            }
            Status::PREPARED | Status::ROLLBACK_ONLY => {
                trace!("rollback() PREPARED or ROLLBACK_ONLY -> rm_rollback()");
                self.rm_rollback(current_gtid)?;
            }
            _ => {}
        }
        self.status = Status::ROLLEDBACK;
        Ok(())
    }

    fn set_transaction_rollbackonly(&mut self) -> Result<(), XaError> {
        self.validate_and_set_status(
            Status::IDLE | Status::ACTIVE | Status::PREPARED | Status::ROLLBACK_ONLY,
            Status::ROLLBACK_ONLY,
        )
    }

    fn get_status(&mut self) -> Result<Status, XaError> {
        Ok(self.status)
    }
}

impl Drop for SimpleTransactionManager {
    fn drop(&mut self) {
        trace!("Drop of SimpleTransactionManager");
        if (Status::ACTIVATING
            | Status::ACTIVE
            | Status::PREPARING
            | Status::PREPARED
            | Status::ROLLBACK_ONLY
            | Status::ROLLINGBACK)
            .contains(self.status)
        {
            let gtid = self.current_gtid.unwrap_or_default();
            self.rm_rollback(gtid).ok();
        }
    }
}
