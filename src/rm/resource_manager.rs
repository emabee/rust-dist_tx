use std::hash::Hash;
use rm::flags::Flags;
use rm::RmResult;
use tm::TransactionId;

/// Interface of a resource manager, as required by a transaction manager.
pub trait ResourceManager: Hash + Eq {
    /// Starts or resumes the work on behalf of a given transaction branch.
    ///
    /// # Arguments
    ///
    /// * `xid` - The id of the transaction branch.
    /// * `flag` - One of Flag::NoFlag, Flag::Join, Flag::Resume.
    fn start(&mut self, id: &TransactionId, flag: Flags) -> RmResult<()>;

    /// Ends work on behalf of a given transaction branch.
    ///
    /// # Arguments
    ///
    /// * `xid` - The id of the transaction branch.
    /// * `flag` - One of Flags::SUCCESS, Flags::FAIL, or Flags::SUSPEND.
    fn end(&mut self, id: &TransactionId, flag: Flags) -> RmResult<()>;

    /// Prepare to commit the work done in the given transaction branch.
    ///
    /// # Arguments
    ///
    /// * `xid` - The id of the transaction branch.
    fn prepare(&mut self, id: &TransactionId) -> RmResult<()>;

    /// Commit the work done in the given transaction branch.
    ///
    /// # Arguments
    ///
    /// * `xid` - The id of the transaction branch.
    fn commit(&mut self, id: &TransactionId) -> RmResult<()>;

    /// Rollback the work done in the given transaction branch.
    ///
    /// # Arguments
    ///
    /// * `xid` - The id of the transaction branch.
    fn rollback(&mut self, id: &TransactionId) -> RmResult<()>;

    /// Tells the server to forget about a heuristically completed transaction.
    ///
    /// # Arguments
    ///
    /// * `xid` - The id of the transaction branch.
    fn forget(&mut self, id: &TransactionId) -> RmResult<()>;

    /// Returns a list of transactions that have been prepared or heuristically completed.
    ///
    /// # Arguments
    ///
    /// * `flag` - One of Flags::START_RECOVERY_SCAN, Flags::END_RECOVERY_SCAN, or none.
    fn recover(&mut self, flag: Flags) -> RmResult<Vec<TransactionId>>;
}
