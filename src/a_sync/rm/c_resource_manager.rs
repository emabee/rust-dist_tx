use crate::{Flags, ReturnCode, RmError, XaTransactionId};

use async_trait::async_trait;

/// Interface of a resource manager that is close to the XA standard.
///
#[async_trait]
pub trait CResourceManager {
    /// Tells the server to start or resume the work on behalf of a given transaction branch.
    ///
    /// # Arguments
    ///
    /// * `xid` - The id of the transaction branch.
    /// * `flag` - One of `Flag::default()`, `Flag::JOIN`, `Flag::RESUME`.
    ///
    /// # Errors
    ///
    /// `RmError` if the request cannot be handled regularily.
    async fn start(&mut self, id: XaTransactionId, flag: Flags) -> Result<ReturnCode, RmError>;

    /// Tells the server to end work on behalf of a given transaction branch.
    ///
    /// # Arguments
    ///
    /// * `xid` - The id of the transaction branch.
    /// * `flag` - One of `Flags::SUCCESS`, `Flags::FAIL`, or `Flags::SUSPEND`.
    ///
    /// # Errors
    ///
    /// `RmError` if the request cannot be handled regularily.
    async fn end(&mut self, id: XaTransactionId, flag: Flags) -> Result<ReturnCode, RmError>;

    /// Tells the server to prepare to commit the work done in the given transaction branch.
    ///
    /// # Arguments
    ///
    /// * `xid` - The id of the transaction branch.
    ///
    /// # Errors
    ///
    /// `RmError` if the request cannot be handled regularily.
    async fn prepare(&mut self, id: XaTransactionId) -> Result<ReturnCode, RmError>;

    /// Tells the server to commit the work done in the given transaction branch.
    ///
    /// # Arguments
    ///
    /// * `xid` - The id of the transaction branch.
    /// * `flag` - One of `Flags::ONE_PHASE`, `Flags::default()`.
    ///
    /// # Errors
    ///
    /// `RmError` if the request cannot be handled regularily.
    async fn commit(&mut self, id: XaTransactionId, flag: Flags) -> Result<ReturnCode, RmError>;

    /// Tells the server to roll back the work done in the given transaction branch.
    ///
    /// # Arguments
    ///
    /// * `xid` - The id of the transaction branch.
    ///
    /// # Errors
    ///
    /// `RmError` if the request cannot be handled regularily.
    async fn rollback(&mut self, id: XaTransactionId) -> Result<ReturnCode, RmError>;

    /// Tells the server to forget about a heuristically completed transaction.
    ///
    /// # Arguments
    ///
    /// * `xid` - The id of the transaction branch.
    ///
    /// # Errors
    ///
    /// `RmError` if the request cannot be handled regularily.
    async fn forget(&mut self, id: XaTransactionId) -> Result<ReturnCode, RmError>;

    /// Returns a list of transactions that have been prepared or heuristically completed.
    ///
    /// # Arguments
    ///
    /// * `flag` - One of `Flags::START_RECOVERY_SCAN`, `Flags::END_RECOVERY_SCAN`, or none.
    ///
    /// # Errors
    ///
    /// `RmError` if the request cannot be handled regularily.
    async fn recover(&mut self, flag: Flags) -> Result<Vec<XaTransactionId>, RmError>;
}
