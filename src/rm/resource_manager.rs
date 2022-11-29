use crate::rm::{Error, ReturnCode};
use crate::tm::XaTransactionId;

/// Interface of a resource manager, as required by a transaction manager.
///
pub trait ResourceManager: std::fmt::Debug {
    /// Tells the server to start work on behalf of the given transaction branch.
    ///
    /// # Errors
    ///
    /// `Error` if the request cannot be handled regularily.
    fn start(&mut self, id: &XaTransactionId) -> Result<ReturnCode, Error>;

    /// Tells the server to join working on behalf of the given transaction branch.
    ///
    /// # Errors
    ///
    /// `Error` if the request cannot be handled regularily.
    fn start_by_joining(&mut self, id: &XaTransactionId) -> Result<ReturnCode, Error>;

    /// Tells the server to resume working on behalf of the given transaction branch.
    ///
    /// # Errors
    ///
    /// `Error` if the request cannot be handled regularily.
    fn start_by_resuming(&mut self, id: &XaTransactionId) -> Result<ReturnCode, Error>;

    /// Tells the server to end working on behalf of the given transaction branch.
    ///
    /// # Errors
    ///
    /// `Error` if the request cannot be handled regularily.
    fn end_success(&mut self, id: &XaTransactionId) -> Result<ReturnCode, Error>;

    /// Tells the server to stop working on behalf of the given transaction branch, transaction
    /// will not be committed.
    ///
    /// # Errors
    ///
    /// `Error` if the request cannot be handled regularily.
    fn end_failure(&mut self, id: &XaTransactionId) -> Result<ReturnCode, Error>;

    /// Tells the server to suspend working on behalf of the given transaction branch.
    ///
    /// # Errors
    ///
    /// `Error` if the request cannot be handled regularily.
    fn end_suspend(&mut self, id: &XaTransactionId) -> Result<ReturnCode, Error>;

    /// Tells the server to prepare to commit the work done in the given transaction branch.
    ///
    /// # Errors
    ///
    /// `Error` if the request cannot be handled regularily.
    fn prepare(&mut self, id: &XaTransactionId) -> Result<ReturnCode, Error>;

    /// Tells the server to commit the work done in the given prepared transaction branch.
    ///
    /// # Errors
    ///
    /// `Error` if the request cannot be handled regularily.
    fn commit(&mut self, id: &XaTransactionId) -> Result<ReturnCode, Error>;

    /// Tells the server to commit the work done in the given not-prepared transaction branch.
    ///
    /// # Errors
    ///
    /// `Error` if the request cannot be handled regularily.
    fn commit_one_phase(&mut self, id: &XaTransactionId) -> Result<ReturnCode, Error>;

    /// Tells the server to rollback the work done in the given transaction branch.
    ///
    /// # Errors
    ///
    /// `Error` if the request cannot be handled regularily.
    fn rollback(&mut self, id: &XaTransactionId) -> Result<ReturnCode, Error>;

    /// Tells the server to forget about the given heuristically completed transaction.
    ///
    /// # Errors
    ///
    /// `Error` if the request cannot be handled regularily.
    fn forget(&mut self, id: &XaTransactionId) -> Result<ReturnCode, Error>;

    /// Returns the list of transactions that have been prepared or heuristically
    /// completed.
    ///
    /// # Errors
    ///
    /// `Error` if the request cannot be handled regularily.
    fn recover(&mut self) -> Result<Vec<XaTransactionId>, Error>;

    /// Returns the list of transactions that have been prepared or heuristically
    /// completed.
    ///
    /// # Errors
    ///
    /// `Error` if the request cannot be handled regularily.
    fn begin_recover(&mut self) -> Result<Vec<XaTransactionId>, Error>;

    /// Returns the list of transactions that have been prepared or heuristically
    /// completed.
    ///
    /// # Errors
    ///
    /// `Error` if the request cannot be handled regularily.
    fn end_recover(&mut self) -> Result<Vec<XaTransactionId>, Error>;
}
