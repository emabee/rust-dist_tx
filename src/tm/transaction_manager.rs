use rm::ResourceManager;
use tm::transaction::Transaction;

use tm::TmResult;

/// Transaction Manager for distributed transactions.
///
/// Use register()/unregister() to define the set of resource managers
/// you want to (potentially) take part in subsequent transactions.
///
/// Then use `start_transaction()` to start a transaction. The rest is done on the
/// application interfaces of the resource manager and on the transaction object.
///
pub trait TransactionManager<T, RM>
where
    T: Transaction,
    RM: ResourceManager,
{
    /// Register a ResourceManager.
    ///
    fn register(&mut self, rm: RM) -> TmResult<()>;

    /// Unregister a ResourceManager.
    ///
    fn unregister(&mut self, rm: RM) -> TmResult<()>;

    /// Hand out a Transaction object with a fresh global_ta and one branch per registered RM.
    fn start_transaction() -> TmResult<T>;

    /// Set a non-default timeout value for transactions being started subsequently with
    /// `start_transaction()`.
    ///
    /// By default, the transaction manager uses some default value for the transaction timeout.
    /// If seconds is set to 0, the default value is restored.
    ///
    fn set_transaction_timeout(seconds: u32);
}
