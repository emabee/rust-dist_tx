use crate::rm::{CResourceManager, Error, Flags, ResourceManager, ReturnCode};
use crate::tm::XaTransactionId;
use log::trace;

/// Wraps an instance of `CResourceManager` and implements `ResourceManager`.
///
/// For registering an instance of "`YourCResourceManager`" at a transaction manager,
/// just use a `Box<CRmWrapper(YourCResourceManager)>`.
#[derive(Debug)]
pub struct CRmWrapper<T>(pub T);

impl<T: CResourceManager + std::fmt::Debug> ResourceManager for CRmWrapper<T> {
    fn start(&mut self, id: &XaTransactionId) -> Result<ReturnCode, Error> {
        trace!("start() with {:?}", id);
        self.0.start(id, Flags::default())
    }

    fn start_by_joining(&mut self, id: &XaTransactionId) -> Result<ReturnCode, Error> {
        trace!("start_by_joining() with {:?}", id);
        self.0.start(id, Flags::JOIN)
    }

    fn start_by_resuming(&mut self, id: &XaTransactionId) -> Result<ReturnCode, Error> {
        trace!("start_by_resuming() with {:?}", id);
        self.0.start(id, Flags::RESUME)
    }

    fn end_success(&mut self, id: &XaTransactionId) -> Result<ReturnCode, Error> {
        trace!("end_success() with {:?}", id);
        self.0.end(id, Flags::SUCCESS)
    }

    fn end_failure(&mut self, id: &XaTransactionId) -> Result<ReturnCode, Error> {
        trace!("end_failure() with {:?}", id);
        self.0.end(id, Flags::FAIL)
    }

    fn end_suspend(&mut self, id: &XaTransactionId) -> Result<ReturnCode, Error> {
        trace!("end_suspend() with {:?}", id);
        self.0.end(id, Flags::SUSPEND)
    }

    fn prepare(&mut self, id: &XaTransactionId) -> Result<ReturnCode, Error> {
        trace!("prepare() with {:?}", id);
        self.0.prepare(id)
    }

    fn commit(&mut self, id: &XaTransactionId) -> Result<ReturnCode, Error> {
        trace!("commit() with {:?}", id);
        self.0.commit(id, Flags::default())
    }

    fn commit_one_phase(&mut self, id: &XaTransactionId) -> Result<ReturnCode, Error> {
        trace!("commit_one_phase() with {:?}", id);
        self.0.commit(id, Flags::ONE_PHASE)
    }

    fn rollback(&mut self, id: &XaTransactionId) -> Result<ReturnCode, Error> {
        trace!("rollback() with {:?}", id);
        self.0.rollback(id)
    }

    fn forget(&mut self, id: &XaTransactionId) -> Result<ReturnCode, Error> {
        trace!("forget() with {:?}", id);
        self.0.forget(id)
    }

    fn recover(&mut self) -> Result<Vec<XaTransactionId>, Error> {
        trace!("recover()");
        self.0
            .recover(Flags::START_RECOVERY_SCAN | Flags::END_RECOVERY_SCAN)
    }

    fn begin_recover(&mut self) -> Result<Vec<XaTransactionId>, Error> {
        trace!("begin_recover()");
        self.0.recover(Flags::START_RECOVERY_SCAN)
    }

    fn end_recover(&mut self) -> Result<Vec<XaTransactionId>, Error> {
        trace!("end_recover()");
        self.0.recover(Flags::END_RECOVERY_SCAN)
    }
}
