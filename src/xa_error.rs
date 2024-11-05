use crate::RmError;
use thiserror::Error;

/// Error of Transaction Manager.
#[derive(Debug, Error)]
pub enum XaError {
    /// Error was caused by one or multiple Resource Manager Errors.
    #[error("Error was caused by one or multiple Resource Manager Errors")]
    RmErrors(Vec<RmError>),
    /// Error was caused by wrong methods calls (wrong state, or wrong parameters).
    #[error("Error was caused by wrong methods calls (wrong state, or wrong parameters)")]
    Usage(&'static str),
    /// Error was caused by wrong methods calls (wrong state, or wrong parameters).
    #[error("Error was caused by wrong methods calls (wrong state, or wrong parameters)")]
    UsageDetails(String),
    /// Some resource managers responded with unexpected errors,
    /// leaving the whole system potentially in an inconsistent state.
    #[error("Some resource managers responded with unexpected errors")]
    Inconsistency(String, Vec<RmError>),
    /// Reading an `XaTransactionId` from a byte stream failed.
    #[error("Reading an XaTransactionId from a byte stream failed")]
    ReadXid(String),
}
impl From<std::io::Error> for XaError {
    fn from(e: std::io::Error) -> XaError {
        XaError::ReadXid(e.to_string())
    }
}
