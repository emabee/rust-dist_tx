/// Return codes used by resource managers.
#[derive(Clone, Debug)]
pub enum RmRc {
    /// A rollback was caused by an unspecified reason.
    RollbackUnspecified,
    /// A rollback was caused by a communication failure.
    RollbackCommunicationFailure,
    /// A deadlock was detected.
    RollbackDeadlock,
    /// A condition that violates the integrity of the resources was detected.
    RollbackIntegrity,
    /// The transaction branch was rolled back for a reason not on this list.
    RollbackOther,
    /// A protocol error occurred in the resource manager.
    RollbackProtocol,
    /// A transaction branch took too long.
    RollbackTimeout,
    /// Transient issue, a retry with this transaction branch may work.
    RollbackTransient,

    /// The transaction branch has been heuristically completed.
    HeuristicallyCompleted,
    /// The transaction branch has been heuristically committed.
    HeuristicallyCommitted,
    /// The transaction branch has been heuristically rolled back.
    HeuristicallyRolledBack,
    /// The transaction branch has been heuristically committed and rolled back.
    HeuristicallyMessedUp,

    /// Nothing has happened, action may be retried.
    Retry,
    /// The transaction branch was read-only and has been committed.
    ReadOnlyCommitted,

    /// Normal execution.
    Ok,

    /// Should never be used.
    UnknownErrorCode(i32),
}
impl RmRc {
    /// Instantiate from the error code as defined in the XA standard.
    pub fn from_i32(i: i32) -> RmRc {
        match i {
            100 => RmRc::RollbackUnspecified,
            101 => RmRc::RollbackCommunicationFailure,
            102 => RmRc::RollbackDeadlock,
            103 => RmRc::RollbackIntegrity,
            104 => RmRc::RollbackOther,
            105 => RmRc::RollbackProtocol,
            106 => RmRc::RollbackTimeout,
            107 => RmRc::RollbackTransient,

            8 => RmRc::HeuristicallyCompleted,
            7 => RmRc::HeuristicallyCommitted,
            6 => RmRc::HeuristicallyRolledBack,
            5 => RmRc::HeuristicallyMessedUp,

            4 => RmRc::Retry,
            3 => RmRc::ReadOnlyCommitted,
            0 => RmRc::Ok,
            i => RmRc::UnknownErrorCode(i),
        }
    }
}

/// Represents the possible errors that can occur in a `ResourceManager`.
#[derive(Debug)]
pub struct RmError {
    c: ErrorCode,
    s: String,
}
impl RmError {
    /// Factory method.
    pub fn new(c: ErrorCode, s: String) -> RmError {
        RmError { c, s }
    }
    /// Returns the kind of error that has occured.
    pub fn get_code(&self) -> ErrorCode {
        self.c.clone()
    }
    /// Returns a textual description of the error.
    pub fn get_description(&self) -> String {
        self.s.clone()
    }
}

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

/// An abbreviation of <code>Result&lt;T, `XaError`&gt;</code>.
///
/// Just for convenience.
pub type RmResult<T> = Result<T, RmError>;
