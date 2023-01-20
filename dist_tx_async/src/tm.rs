//! Transactions and the transaction manager trait, and a simple implementation.
mod simple_transaction_manager;
mod transaction_manager;

pub use self::{
    simple_transaction_manager::SimpleTransactionManager, transaction_manager::Status,
    transaction_manager::TransactionManager,
};
