use rm::flags::Flags;
use rm::RmResult;
use tm::XaTransactionId;

/// Interface of a resource manager that is close to the XA standard.
///
pub trait CResourceManager {
    /// Tells the server to start or resume the work on behalf of a given transaction branch.
    ///
    /// # Arguments
    ///
    /// * `xid` - The id of the transaction branch.
    /// * `flag` - One of Flag::default(), Flag::JOIN, Flag::RESUME.
    fn start(&mut self, id: &XaTransactionId, flag: Flags) -> RmResult<()>;

    /// Tells the server to end work on behalf of a given transaction branch.
    ///
    /// # Arguments
    ///
    /// * `xid` - The id of the transaction branch.
    /// * `flag` - One of Flags::SUCCESS, Flags::FAIL, or Flags::SUSPEND.
    fn end(&mut self, id: &XaTransactionId, flag: Flags) -> RmResult<()>;

    /// Tells the server to prepare to commit the work done in the given transaction branch.
    ///
    /// # Arguments
    ///
    /// * `xid` - The id of the transaction branch.
    fn prepare(&mut self, id: &XaTransactionId) -> RmResult<()>;

    /// Tells the server to commit the work done in the given transaction branch.
    ///
    /// # Arguments
    ///
    /// * `xid` - The id of the transaction branch.
    /// * `flag` - One of Flags::ONE_PHASE, Flags::default().
    fn commit(&mut self, id: &XaTransactionId, flag: Flags) -> RmResult<()>;

    /// Tells the server to roll back the work done in the given transaction branch.
    ///
    /// # Arguments
    ///
    /// * `xid` - The id of the transaction branch.
    fn rollback(&mut self, id: &XaTransactionId) -> RmResult<()>;

    /// Tells the server to forget about a heuristically completed transaction.
    ///
    /// # Arguments
    ///
    /// * `xid` - The id of the transaction branch.
    fn forget(&mut self, id: &XaTransactionId) -> RmResult<()>;

    /// Returns a list of transactions that have been prepared or heuristically completed.
    ///
    /// # Arguments
    ///
    /// * `flag` - One of Flags::START_RECOVERY_SCAN, Flags::END_RECOVERY_SCAN, or none.
    fn recover(&mut self, flag: Flags) -> RmResult<Vec<XaTransactionId>>;
}
