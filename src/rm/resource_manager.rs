use rm::rm_error::RmRc;
use rm::RmResult;
use tm::XaTransactionId;

/// Interface of a resource manager, as required by a transaction manager.
///
pub trait ResourceManager {
    /// Tells the server to start work on behalf of the given transaction branch.
    fn start(&mut self, id: &XaTransactionId) -> RmResult<RmRc>;

    /// Tells the server to join working on behalf of the given transaction branch.
    fn start_by_joining(&mut self, id: &XaTransactionId) -> RmResult<RmRc>;

    /// Tells the server to resume working on behalf of the given transaction branch.
    fn start_by_resuming(&mut self, id: &XaTransactionId) -> RmResult<RmRc>;


    /// Tells the server to end working on behalf of the given transaction branch.
    fn end_success(&mut self, id: &XaTransactionId) -> RmResult<RmRc>;

    /// Tells the server to stop working on behalf of the given transaction branch, transaction
    /// will not be committed.
    fn end_failure(&mut self, id: &XaTransactionId) -> RmResult<RmRc>;

    /// Tells the server to suspend working on behalf of the given transaction branch.
    fn end_suspend(&mut self, id: &XaTransactionId) -> RmResult<RmRc>;


    /// Tells the server to prepare to commit the work done in the given transaction branch.
    fn prepare(&mut self, id: &XaTransactionId) -> RmResult<RmRc>;


    /// Tells the server to commit the work done in the given prepared transaction branch.
    fn commit(&mut self, id: &XaTransactionId) -> RmResult<RmRc>;

    /// Tells the server to commit the work done in the given not-prepared transaction branch.
    fn commit_one_phase(&mut self, id: &XaTransactionId) -> RmResult<RmRc>;

    /// Tells the server to rollback the work done in the given transaction branch.
    fn rollback(&mut self, id: &XaTransactionId) -> RmResult<RmRc>;


    /// Tells the server to forget about the given heuristically completed transaction.
    fn forget(&mut self, id: &XaTransactionId) -> RmResult<RmRc>;

    /// Returns the list of transactions that have been prepared or heuristically
    /// completed.
    fn recover(&mut self) -> RmResult<Vec<XaTransactionId>>;

    /// Returns the list of transactions that have been prepared or heuristically
    /// completed.
    fn begin_recover(&mut self) -> RmResult<Vec<XaTransactionId>>;

    /// Returns the list of transactions that have been prepared or heuristically
    /// completed.
    fn end_recover(&mut self) -> RmResult<Vec<XaTransactionId>>;
}
