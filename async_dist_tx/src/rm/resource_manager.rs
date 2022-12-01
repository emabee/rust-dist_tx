use crate::{ReturnCode, RmError, XaTransactionId};
use async_trait::async_trait;

/// Interface of a resource manager, as required by a transaction manager.
///
#[async_trait]
pub trait ResourceManager: std::fmt::Debug + Send {
    /// Tells the server to start work on behalf of the given transaction branch.
    ///
    /// # Errors
    ///
    /// `RmError` if the request cannot be handled regularily.
    async fn start(&mut self, id: XaTransactionId) -> Result<ReturnCode, RmError>;

    /// Tells the server to join working on behalf of the given transaction branch.
    ///
    /// # Errors
    ///
    /// `RmError` if the request cannot be handled regularily.
    async fn start_by_joining(&mut self, id: XaTransactionId) -> Result<ReturnCode, RmError>;

    /// Tells the server to resume working on behalf of the given transaction branch.
    ///
    /// # Errors
    ///
    /// `RmError` if the request cannot be handled regularily.
    async fn start_by_resuming(&mut self, id: XaTransactionId) -> Result<ReturnCode, RmError>;

    /// Tells the server to end working on behalf of the given transaction branch.
    ///
    /// # Errors
    ///
    /// `RmError` if the request cannot be handled regularily.
    async fn end_success(&mut self, id: XaTransactionId) -> Result<ReturnCode, RmError>;

    /// Tells the server to stop working on behalf of the given transaction branch, transaction
    /// will not be committed.
    ///
    /// # Errors
    ///
    /// `RmError` if the request cannot be handled regularily.
    async fn end_failure(&mut self, id: XaTransactionId) -> Result<ReturnCode, RmError>;

    /// Tells the server to suspend working on behalf of the given transaction branch.
    ///
    /// # Errors
    ///
    /// `RmError` if the request cannot be handled regularily.
    async fn end_suspend(&mut self, id: XaTransactionId) -> Result<ReturnCode, RmError>;

    /// Tells the server to prepare to commit the work done in the given transaction branch.
    ///
    /// # Errors
    ///
    /// `RmError` if the request cannot be handled regularily.
    async fn prepare(&mut self, id: XaTransactionId) -> Result<ReturnCode, RmError>;

    /// Tells the server to commit the work done in the given prepared transaction branch.
    ///
    /// # Errors
    ///
    /// `RmError` if the request cannot be handled regularily.
    async fn commit(&mut self, id: XaTransactionId) -> Result<ReturnCode, RmError>;

    /// Tells the server to commit the work done in the given not-prepared transaction branch.
    ///
    /// # Errors
    ///
    /// `RmError` if the request cannot be handled regularily.
    async fn commit_one_phase(&mut self, id: XaTransactionId) -> Result<ReturnCode, RmError>;

    /// Tells the server to rollback the work done in the given transaction branch.
    ///
    /// # Errors
    ///
    /// `RmError` if the request cannot be handled regularily.
    async fn rollback(&mut self, id: XaTransactionId) -> Result<ReturnCode, RmError>;

    /// Tells the server to forget about the given heuristically completed transaction.
    ///
    /// # Errors
    ///
    /// `RmError` if the request cannot be handled regularily.
    async fn forget(&mut self, id: XaTransactionId) -> Result<ReturnCode, RmError>;

    /// Returns the list of transactions that have been prepared or heuristically
    /// completed.
    ///
    /// # Errors
    ///
    /// `RmError` if the request cannot be handled regularily.
    async fn recover(&mut self) -> Result<Vec<XaTransactionId>, RmError>;

    /// Returns the list of transactions that have been prepared or heuristically
    /// completed.
    ///
    /// # Errors
    ///
    /// `RmError` if the request cannot be handled regularily.
    async fn begin_recover(&mut self) -> Result<Vec<XaTransactionId>, RmError>;

    /// Returns the list of transactions that have been prepared or heuristically
    /// completed.
    ///
    /// # Errors
    ///
    /// `RmError` if the request cannot be handled regularily.
    async fn end_recover(&mut self) -> Result<Vec<XaTransactionId>, RmError>;
}
