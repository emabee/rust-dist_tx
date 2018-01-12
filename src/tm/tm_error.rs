use rm::RmError;

/// Abbreviation for `Result<T, TmError>`.
pub type TmResult<T> = Result<T, TmError>;

/// Error of Transaction Manager.
pub enum TmError {
    /// Error was caused by one or multiple Resource Manager Errors.
    RmError(Vec<RmError>),
    /// Error was caused by wrong methods calls (wrong state, or wrong parameters).
    Usage(&'static str),
}
