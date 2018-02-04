use rm::RmRc;
use std::collections::HashMap;
use rm::RmResult;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::u64;
use tm::{TmStatus, TransactionManager, XaError, XaResult, XaTransactionId};
use rm::{ResourceManager, RmError};
use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use std::io::Cursor;

/// The format id that is used for the `XaTransactionId`s of `SimpleTransactionManager`.
const FORMAT_ID: i32 = 99;

/// `SimpleTransactionManager`
///
/// * identifies itself with a `u64` (hash of its name)
/// * identifies the resource managers with a `u64` given during the registration
/// * enumerates its global transactions with a `u64` counter, starting with 0, or,
///   if higher values are found during rm-registration, with the highest found number (plus 1)
///
/// uses `XaTransactionId` with
///
/// * `format_id` = 99
/// * `global_tid`: u64 counter, starting from 0
/// * `branch_qualifier`: `tm_id`: u64, `rm_id`: u64
///
/// A minimal implementation of the `TransactionManager` interface.
///
/// Is identified with an application-defined String, whose hash is used as `tm_id`.
///
/// No support is provided for multi-threading á la XA.
///
pub struct SimpleTransactionManager {
    name: String,
    id: u64,
    rms: HashMap<u64, Box<ResourceManager>>,
    last_gtid: u64,
    current_gtid: Option<u64>,
    status: TmStatus,
}
impl SimpleTransactionManager {
    /// Produces a new instance.
    pub fn new(name: String) -> SimpleTransactionManager {
        trace!("new()");
        let mut s = DefaultHasher::new();
        name.hash(&mut s);
        SimpleTransactionManager {
            name: name,
            id: s.finish() & (u64::MAX - 0b_1111_1111_u64),
            rms: HashMap::<u64, Box<ResourceManager>>::new(),
            last_gtid: 0,
            current_gtid: None,
            status: TmStatus::IDLE,
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

    fn get_current_gtid(&mut self) -> XaResult<u64> {
        match self.current_gtid {
            None => Err(XaError::Usage("No current transaction set")),
            Some(u) => Ok(u),
        }
    }

    fn validate_and_set_status(&mut self, required: TmStatus, new: TmStatus) -> XaResult<()> {
        if self.status != required {
            Err(XaError::UsageDetails(format!(
                "SimpleTransactionManager is in state {:?}, but state {:?} is required",
                self.status,
                required
            )))
        } else {
            self.status = new;
            Ok(())
        }
    }

    /// Reports the name of this instance.
    pub fn name(&self) -> &str {
        &self.name
    }


    fn rm_action<F>(&mut self, action: F, global_tid: &u64) -> XaResult<()>
    where
        F: Fn(&mut Box<ResourceManager>, &XaTransactionId) -> RmResult<RmRc>,
    {
        let mut errors = Vec::<RmError>::new();
        for (rm_id, rm) in &mut self.rms {
            let xatid = new_xatid(global_tid, &self.id, rm_id);
            if let Err(e) = action(rm, &xatid) {
                errors.push(e)
            }
        }
        if errors.is_empty() {
            Ok(())
        } else {
            Err(XaError::RmErrors(errors))
        }
    }

    fn rm_start(&mut self, global_tid: &u64) -> XaResult<()> {
        self.rm_action(|rm, xatid| ((**rm).start(xatid)), global_tid)
    }

    // fn rm_join(&mut self, global_tid: &u64) -> XaResult<()> {
    //     self.rm_action(|rm, xatid| ((**rm).start_by_joining(xatid)), global_tid)
    // }

    // fn rm_resume(&mut self, global_tid: &u64) -> XaResult<()> {
    //     self.rm_action(|rm, xatid| ((**rm).start_by_resuming(xatid)), global_tid)
    // }

    // fn rm_suspend(&mut self, global_tid: &u64) -> XaResult<()> {
    //     self.rm_action(|rm, xatid| ((**rm).end_suspend(xatid)), global_tid)
    // }

    fn rm_end_success(&mut self, global_tid: &u64) -> XaResult<()> {
        self.rm_action(|rm, xatid| ((**rm).end_success(xatid)), global_tid)
    }

    fn rm_end_failure(&mut self, global_tid: &u64) -> XaResult<()> {
        self.rm_action(|rm, xatid| ((**rm).end_failure(xatid)), global_tid)
    }

    fn rm_prepare(&mut self, global_tid: &u64) -> XaResult<()> {
        self.rm_action(|rm, xatid| ((**rm).prepare(xatid)), global_tid)
    }

    fn rm_commit(&mut self, global_tid: &u64) -> XaResult<()> {
        self.rm_action(|rm, xatid| ((**rm).commit(xatid)), global_tid)
    }

    fn rm_commit_one_phase(&mut self, global_tid: &u64) -> XaResult<()> {
        self.rm_action(|rm, xatid| ((**rm).commit_one_phase(xatid)), global_tid)
    }

    fn rm_rollback(&mut self, global_tid: &u64) -> XaResult<()> {
        self.rm_action(|rm, xatid| ((**rm).rollback(xatid)), global_tid)
    }

    // fn rm_forget(&mut self, global_tid: &u64) -> XaResult<()> {
    //     self.rm_action(|rm, xatid| ((**rm).forget(xatid)), global_tid)
    // }

    // fn rm_recover(&mut self) -> XaResult<Vec<XaTransactionId>> {
    //     panic!("not yet implemented")
    // }


    fn trace_error(&self, e: &XaError, gtid: &u64, method_name: &'static str) {
        if let XaError::RmErrors(ref vec_rmerr) = *e {
            for rm in vec_rmerr {
                trace!("{}({}) failed due to {:?}", method_name, gtid, rm);
            }
        } else {
            trace!("error in {}: {}", method_name, e.get_description());
        }
    }

    fn try_rollback_after(&mut self, current_gtid: &u64, method: &'static str) -> XaResult<()> {
        self.status = TmStatus::ROLLINGBACK;
        let result = self.rm_rollback(current_gtid);
        if let Err(ref e) = result {
            self.trace_error(e, current_gtid, "error in rm_rollback");
        }
        self.status = TmStatus::IDLE;
        match result {
            Err(e) => if let XaError::RmErrors(v) = e {
                Err(XaError::Inconsistency(
                    format!("rm_rollback() failed after a failed {}()", method),
                    v,
                ))
            } else {
                Err(e)
            },
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

fn new_xatid(global_tid: &u64, tm_id: &u64, rm_id: &u64) -> XaTransactionId {
    let mut v_gt: Vec<u8> = vec![];
    v_gt.write_u64::<LittleEndian>(*global_tid).unwrap();
    let mut v_bq: Vec<u8> = vec![];
    v_bq.write_u64::<LittleEndian>(*tm_id).unwrap();
    v_bq.write_u64::<LittleEndian>(*rm_id).unwrap();
    XaTransactionId::new(FORMAT_ID, v_gt, v_bq).unwrap()
}



impl TransactionManager for SimpleTransactionManager {
    fn register(
        &mut self,
        mut rm: Box<ResourceManager>,
        rm_id: u64,
        cleanup: bool,
    ) -> XaResult<()> {
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
                    (*rm).forget(xid).unwrap_or(RmRc::Ok);
                }
            }
        }

        self.rms.insert(rm_id, rm);
        Ok(())
    }

    fn unregister(&mut self, rm_id: u64) -> XaResult<()> {
        self.rms.remove(&rm_id);
        Ok(())
    }

    // Creates a new Global Transaction and tells all rms to start working for a respective branch.
    //
    // TM must be in status TmStatus::Idle.
    // If successful, sets status to TmStatus::Active.
    // If not,???
    fn start_transaction(&mut self) -> XaResult<()> {
        trace!("start_transaction()");
        self.validate_and_set_status(TmStatus::IDLE, TmStatus::ACTIVATING)?;

        let global_tid = self.next_global_tid();

        trace!("start_transaction() -> rm_start({})", global_tid);
        match self.rm_start(&global_tid) {
            Ok(()) => {
                self.current_gtid = Some(global_tid);
                self.status = TmStatus::ACTIVE;
                return Ok(());
            }
            Err(e) => {
                trace!(
                    "start_transaction() -> rm_start({}) failed with {:?}",
                    global_tid,
                    e
                );

                trace!("start_transaction() -> rm_end_failure({})", global_tid);
                if let Err(XaError::RmErrors(v)) = self.rm_end_failure(&global_tid) {
                    trace!(
                        "start_transaction() -> rm_end_failure({}) failed with {:?}",
                        global_tid,
                        v
                    );
                }

                trace!("start_transaction() -> rm_rollback({})", global_tid);
                if let Err(XaError::RmErrors(v)) = self.rm_rollback(&global_tid) {
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
        match self.rm_start(&global_tid) {
            Ok(()) => {
                self.current_gtid = Some(global_tid);
                self.status = TmStatus::ACTIVE;
                Ok(())
            }
            Err(e) => {
                trace!(
                    "start_transaction() -> rm_start({}), second attempt failed, too",
                    global_tid
                );
                self.status = TmStatus::IDLE;
                Err(e)
            }
        }
    }

    // Internally, does commit_one_phase if only a single RM is involved, otherwise does
    // 2PC: (end_success(), preprare(), commit() on all participating RMs)
    // Completes the transaction, if it is in state `TmStatus::Active`.
    //
    // If successful, the transaction is set to state `TmStatus::Committed`, otherwise to
    // `TmStatus::Failed` or `TmStatus::RolledBack`.
    fn commit_transaction(&mut self) -> XaResult<()> {
        trace!("commit()");
        let current_gtid = self.get_current_gtid()?;
        self.validate_and_set_status(TmStatus::ACTIVE, TmStatus::COMMITTING)?;

        // shortcut, if possible
        if self.rms.len() < 2 {
            trace!("commit() -> rm_commit_one_phase()");
            return self.rm_commit_one_phase(&current_gtid);
        }

        // 1. end_success()
        trace!("commit() -> rm_end_success()");
        if let Err(e) = self.rm_end_success(&current_gtid) {
            self.trace_error(&e, &current_gtid, "rm_end_success");
            return self.try_rollback_after(&current_gtid, "rm_end_success");
        }

        // 2. prepare()
        trace!("commit() -> rm_prepare()");
        if let Err(e) = self.rm_prepare(&current_gtid) {
            self.trace_error(&e, &current_gtid, "rm_prepare");
            return self.try_rollback_after(&current_gtid, "rm_prepare");
        }

        // 3. commit()
        trace!("commit() -> rm_commit()");
        if let Err(e) = self.rm_commit(&current_gtid) {
            self.trace_error(&e, &current_gtid, "rm_commit");
            return self.try_rollback_after(&current_gtid, "rm_commit");
        }
        Ok(())
    }

    fn rollback_transaction(&mut self) -> XaResult<()> {
        trace!("rollback()");
        let current_gtid = self.get_current_gtid()?;
        self.validate_and_set_status(
            TmStatus::ACTIVE | TmStatus::IDLE | TmStatus::PREPARED | TmStatus::ROLLBACK_ONLY
                | TmStatus::ROLLEDBACK,
            TmStatus::ROLLINGBACK,
        )?;

        if let Err(e) = self.rm_rollback(&current_gtid) {
            self.trace_error(&e, &current_gtid, "rm_rollback");
            self.try_rollback_after(&current_gtid, "rm_rollback")
        } else {
            Ok(())
        }
    }

    fn set_transaction_rollbackonly(&mut self) -> XaResult<()> {
        self.validate_and_set_status(
            TmStatus::IDLE | TmStatus::ACTIVE | TmStatus::PREPARED | TmStatus::ROLLBACK_ONLY,
            TmStatus::ROLLBACK_ONLY,
        )
    }

    fn get_status(&mut self) -> XaResult<TmStatus> {
        Ok(self.status)
    }
}
