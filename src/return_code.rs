/// Return codes used by resource managers.
#[derive(Clone, Debug)]
pub enum ReturnCode {
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
impl ReturnCode {
    /// Instantiate from the error code as defined in the XA standard.
    #[must_use]
    pub fn from_i32(i: i32) -> ReturnCode {
        match i {
            100 => ReturnCode::RollbackUnspecified,
            101 => ReturnCode::RollbackCommunicationFailure,
            102 => ReturnCode::RollbackDeadlock,
            103 => ReturnCode::RollbackIntegrity,
            104 => ReturnCode::RollbackOther,
            105 => ReturnCode::RollbackProtocol,
            106 => ReturnCode::RollbackTimeout,
            107 => ReturnCode::RollbackTransient,

            8 => ReturnCode::HeuristicallyCompleted,
            7 => ReturnCode::HeuristicallyCommitted,
            6 => ReturnCode::HeuristicallyRolledBack,
            5 => ReturnCode::HeuristicallyMessedUp,

            4 => ReturnCode::Retry,
            3 => ReturnCode::ReadOnlyCommitted,
            0 => ReturnCode::Ok,
            i => ReturnCode::UnknownErrorCode(i),
        }
    }
}
