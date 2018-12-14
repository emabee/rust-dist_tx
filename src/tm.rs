//! Transactions and the transaction manager trait, and a simple implementation.
mod simple_transaction_manager;
mod transaction_manager;
mod xa_error;
mod xa_transaction_id;

pub use self::simple_transaction_manager::SimpleTransactionManager;
pub use self::transaction_manager::TmStatus;
pub use self::transaction_manager::TransactionManager;
pub use self::xa_error::{XaError, XaResult};
pub use self::xa_transaction_id::XaTransactionId;
