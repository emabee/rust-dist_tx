use super::tm_error::TmResult;

pub trait Transaction {
    /// Complete the transaction represented by this Transaction object.
    fn commit() -> TmResult<()>;

    /// Obtain the status of the transaction associated with the target Transaction object.
    fn get_status() -> TmResult<i32>;

    /// Rollback the transaction represented by this Transaction object.
    fn rollback() -> TmResult<()>;

    /// Mark this transaction that its only possible outcome is to be rolled back.
    fn set_rollbackonly() -> TmResult<()>;
}
