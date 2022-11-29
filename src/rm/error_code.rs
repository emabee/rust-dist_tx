/// Errors occuring in resource managers.
#[derive(Clone, Debug)]
pub enum ErrorCode {
    /// A resource manager error occurred in the transaction branch.
    RmError,
    /// Invalid Transaction ID.
    InvalidTransactionId,
    /// Invalid arguments were given.
    InvalidArguments,
    /// routine invoked in an improper context.
    ProtocolError,
    /// resource manager is unavailable.
    RmFailure,
    /// The XID already exists.
    DuplicateTransactionId,
    /// Should never be used.
    UnknownErrorCode(i32),
}
impl ErrorCode {
    /// Instantiate from the error code as defined in the XA standard.
    #[must_use]
    pub fn from_i32(i: i32) -> ErrorCode {
        match i {
            -3 => ErrorCode::RmError,
            -4 => ErrorCode::InvalidTransactionId,
            -5 => ErrorCode::InvalidArguments,
            -6 => ErrorCode::ProtocolError,
            -7 => ErrorCode::RmFailure,
            -8 => ErrorCode::DuplicateTransactionId,
            i => ErrorCode::UnknownErrorCode(i),
        }
    }
}
