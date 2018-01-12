/// Represents the possible errors that can occur.
#[derive(Debug)]
pub struct RmError(pub Kind, pub String);

/// Errors occuring in resource managers.
#[derive(Debug)]
pub enum Kind {
    /// A rollback was caused by an unspecified reason.
    Rollback,
    /// A rollback was caused by a communication failure.
    CommunicationFailure,
    /// A deadlock was detected.
    Deadlock,
    /// A condition that violates the integrity of the resources was detected.
    Integrity,
    /// The resource manager rolled back the transaction branch for a reason not on this list.
    Other,
    /// A protocol error occurred in the resource manager.
    Protocol,
    /// A transaction branch took too long.
    Timeout,
    /// The transaction branch has been heuristically committed.
    HeuristicallyCommitted,
    /// The transaction branch has been heuristically rolled back.
    HeuristicallyRolledBack,
    /// The transaction branch was read-only and has been committed.
    ReadOnlyCommitted,
    /// Invalid Transaction ID.
    InvalidTransactionId,
    /// The XID already exists.
    DuplicateTransactionId,
    /// Error caused by bad usage of API.
    Usage,
}


/// An abbreviation of <code>Result&lt;T, `XaError`&gt;</code>.
///
/// Just for convenience.
pub type RmResult<T> = Result<T, RmError>;
