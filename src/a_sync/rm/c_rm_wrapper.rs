use super::{CResourceManager, ResourceManager};
use crate::{Flags, ReturnCode, RmError, XaTransactionId};
use async_trait::async_trait;
use log::trace;

/// Wraps an instance of `CResourceManager` and implements `ResourceManager`.
///
/// For registering an instance of "`YourCResourceManager`" at a transaction manager,
/// just use a `Box<CRmWrapper(YourCResourceManager)>`.
#[derive(Debug)]
pub struct CRmWrapper<T>(pub T);

#[async_trait]
impl<T: CResourceManager + std::fmt::Debug + std::marker::Send> ResourceManager for CRmWrapper<T> {
    async fn start(&mut self, id: XaTransactionId) -> Result<ReturnCode, RmError> {
        trace!("start() with {:?}", id);
        self.0.start(id, Flags::default()).await
    }

    async fn start_by_joining(&mut self, id: XaTransactionId) -> Result<ReturnCode, RmError> {
        trace!("start_by_joining() with {:?}", id);
        self.0.start(id, Flags::JOIN).await
    }

    async fn start_by_resuming(&mut self, id: XaTransactionId) -> Result<ReturnCode, RmError> {
        trace!("start_by_resuming() with {:?}", id);
        self.0.start(id, Flags::RESUME).await
    }

    async fn end_success(&mut self, id: XaTransactionId) -> Result<ReturnCode, RmError> {
        trace!("end_success() with {:?}", id);
        self.0.end(id, Flags::SUCCESS).await
    }

    async fn end_failure(&mut self, id: XaTransactionId) -> Result<ReturnCode, RmError> {
        trace!("end_failure() with {:?}", id);
        self.0.end(id, Flags::FAIL).await
    }

    async fn end_suspend(&mut self, id: XaTransactionId) -> Result<ReturnCode, RmError> {
        trace!("end_suspend() with {:?}", id);
        self.0.end(id, Flags::SUSPEND).await
    }

    async fn prepare(&mut self, id: XaTransactionId) -> Result<ReturnCode, RmError> {
        trace!("prepare() with {:?}", id);
        self.0.prepare(id).await
    }

    async fn commit(&mut self, id: XaTransactionId) -> Result<ReturnCode, RmError> {
        trace!("commit() with {:?}", id);
        self.0.commit(id, Flags::default()).await
    }

    async fn commit_one_phase(&mut self, id: XaTransactionId) -> Result<ReturnCode, RmError> {
        trace!("commit_one_phase() with {:?}", id);
        self.0.commit(id, Flags::ONE_PHASE).await
    }

    async fn rollback(&mut self, id: XaTransactionId) -> Result<ReturnCode, RmError> {
        trace!("rollback() with {:?}", id);
        self.0.rollback(id).await
    }

    async fn forget(&mut self, id: XaTransactionId) -> Result<ReturnCode, RmError> {
        trace!("forget() with {:?}", id);
        self.0.forget(id).await
    }

    async fn recover(&mut self) -> Result<Vec<XaTransactionId>, RmError> {
        trace!("recover()");
        self.0
            .recover(Flags::START_RECOVERY_SCAN | Flags::END_RECOVERY_SCAN)
            .await
    }

    async fn begin_recover(&mut self) -> Result<Vec<XaTransactionId>, RmError> {
        trace!("begin_recover()");
        self.0.recover(Flags::START_RECOVERY_SCAN).await
    }

    async fn end_recover(&mut self) -> Result<Vec<XaTransactionId>, RmError> {
        trace!("end_recover()");
        self.0.recover(Flags::END_RECOVERY_SCAN).await
    }
}
