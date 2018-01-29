/// Represents the possible errors that can occur in a `ResourceManager`.
#[derive(Debug)]
pub struct RmError {
    k: Kind,
    s: String,
}
impl RmError {
    /// Factory method.
    pub fn new(k: Kind, s: String) -> RmError {
        RmError { k: k, s: s }
    }
    /// Returns the kind of error that has occured.
    pub fn get_kind(&self) -> Kind {
        self.k.clone()
    }
    /// Returns a textual description of the error.
    pub fn get_description(&self) -> String {
        self.s.clone()
    }
}

/// Errors occuring in resource managers.
#[derive(Clone, Debug)]
pub enum Kind {
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
impl Kind {
    /// Instantiate from the error code as defined in the XA standard.
    pub fn from_i32(i: i32) -> Kind {
        match i {
            100 => Kind::RollbackUnspecified,
            101 => Kind::RollbackCommunicationFailure,
            102 => Kind::RollbackDeadlock,
            103 => Kind::RollbackIntegrity,
            104 => Kind::RollbackOther,
            105 => Kind::RollbackProtocol,
            106 => Kind::RollbackTimeout,
            107 => Kind::RollbackTransient,

            8 => Kind::HeuristicallyCompleted,
            7 => Kind::HeuristicallyCommitted,
            6 => Kind::HeuristicallyRolledBack,
            5 => Kind::HeuristicallyMessedUp,

            4 => Kind::Retry,
            3 => Kind::ReadOnlyCommitted,

            -3 => Kind::RmError,
            -4 => Kind::InvalidTransactionId,
            -5 => Kind::InvalidArguments,
            -6 => Kind::ProtocolError,
            -7 => Kind::RmFailure,
            -8 => Kind::DuplicateTransactionId,
            i => Kind::UnknownErrorCode(i),
        }
    }
}

/// An abbreviation of <code>Result&lt;T, `XaError`&gt;</code>.
///
/// Just for convenience.
pub type RmResult<T> = Result<T, RmError>;
