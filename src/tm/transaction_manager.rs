use rm::ResourceManager;
use tm::XaResult;

/// A transaction manager for distributed transactions.
///
/// Use register()/unregister() to define the set of resource managers
/// you want to (potentially) take part in subsequent transactions.
///
/// Then use `start_transaction()` to start a transaction. The rest is done on the
/// application interfaces of the resource manager and on the transaction object.
///
///
pub trait TransactionManager {
    /// Register a ResourceManager.
    ///
    /// Note that `Box<CResourceManager>` also implements `ResourceManager`, so you can hand in
    /// here a `Box<Box<ResourceManagerImpl>>`.
    /// Note that each registration must use a different rm_id - overwrites will not be allowed.
    fn register(&mut self, rm: Box<ResourceManager>, rm_id: u64, cleanup: bool) -> XaResult<()>;

    /// Unregister a ResourceManager.
    fn unregister(&mut self, rm_id: u64) -> XaResult<()>;

    /// Starts a new transaction with a fresh global TA ID and one branch per registered RM.
    ///
    /// The method fails if the last transaction is not yet completed.
    fn start_transaction(&mut self) -> XaResult<()>;

    // /// Obtains a list of open Transactions-IDs from the registered resource managers.
    //     fn recover() -> XaResult<Vec<T>>;
    // FIXME we'll need a function to continue with one of these XaTransactions.

    // /// Sets a non-default timeout value for transactions being started subsequently with
    // /// `start_transaction()`.
    // ///
    // /// By default, the transaction manager uses some default value for the transaction timeout.
    // /// If seconds is set to 0, the default value is restored.
    // ///
    // fn set_transaction_timeout(seconds: u32);

    // Internally, does commit_one_phase if only a single RM is involved, otherwise does
    // 2PC: (end_success(), preprare(), commit() on all participating RMs)
    /// Completes the transaction, if it is in state `TmStatus::Active`.
    ///
    /// If successful, the transaction is set to state `TmStatus::Committed`, otherwise to
    /// `TmStatus::Failed` or `TmStatus::RolledBack`.
    fn commit_transaction(&mut self) -> XaResult<()>;

    /// Rolls the transaction back, discarding all changes, and setting the status to
    /// `TmStatus::RolledBack`.
    fn rollback_transaction(&mut self) -> XaResult<()>;

    /// Mark the transaction that its only possible outcome is to be rolled back.
    fn set_transaction_rollbackonly(&mut self) -> XaResult<()>;

    /// Returns the status of the transaction.
    fn get_status(&mut self) -> XaResult<TmStatus>;
}



bitflags! {
    /// States of a `TransactionManager`.
    #[derive(Default)]
    pub struct TmStatus: u32 {
        /// No transaction in use.
        const IDLE = 0x00_00_00_01;

        /// New Transaction is currently being started.
        const ACTIVATING = 0x00_00_00_02;

        /// Current transaction can be used for changes.
        const ACTIVE = 0x00_00_00_04;

        /// Current transaction is currently being prepared.
        const PREPARING = 0x00_00_00_08;

        /// Current transaction is ready to be committed and can no more be used for changes.
        const PREPARED = 0x00_00_01_00;

        /// An attempt to commit the current transaction is ongoing.
        const COMMITTING = 0x00_00_02_00;

        /// Current transaction was successfully committed.
        const COMMITTED = 0x00_00_04_00;

        /// Current transaction has failed or was marked as RollbackOnly and cannot be committed.
        const ROLLBACK_ONLY = 0x00_00_08_00;

        /// Current transaction is currently being rolled back.
        const ROLLINGBACK = 0x00_01_00_00;

        /// Current transaction is rolled back.
        const ROLLEDBACK = 0x00_02_00_00;
    }
}
