use crate::rm::RmError;
use std::error::Error;
use std::io;

/// Abbreviation for `Result<T, XaError>`.
pub type XaResult<T> = Result<T, XaError>;

/// Error of Transaction Manager.
#[derive(Debug)]
pub enum XaError {
    /// Error was caused by one or multiple Resource Manager Errors.
    RmErrors(Vec<RmError>),
    /// Error was caused by wrong methods calls (wrong state, or wrong parameters).
    Usage(&'static str),
    /// Error was caused by wrong methods calls (wrong state, or wrong parameters).
    UsageDetails(String),
    /// Some Resource Managers responded with unexpected errors,
    /// leaving the whole system potentially in an inconsistent state.
    Inconsistency(String, Vec<RmError>),
    /// Reading an XaTransactionId from a byte stream failed.
    ReadXid(String),
}
impl XaError {
    /// Returns a textual description of the error.
    pub fn get_description(&self) -> String {
        match *self {
            XaError::RmErrors(_) => {
                "Error was caused by one or multiple Resource Manager Errors".to_owned()
            }
            XaError::Usage(s) => s.to_string(),
            XaError::UsageDetails(ref s)
            | XaError::ReadXid(ref s)
            | XaError::Inconsistency(ref s, _) => s.clone(),
        }
    }
}

impl From<io::Error> for XaError {
    fn from(e: io::Error) -> XaError {
        XaError::ReadXid(e.description().to_owned())
    }
}
