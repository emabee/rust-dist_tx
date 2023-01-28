use super::error_code::ErrorCode;

/// Represents the possible errors that can occur in a `ResourceManager` (
/// [`sync::rm::ResourceManager`](sync/rm/trait.ResourceManager.html) and
/// [`a_sync::rm::ResourceManager`](a_sync/rm/trait.ResourceManager.html)
/// ).
#[derive(Debug)]
pub struct RmError {
    c: ErrorCode,
    s: String,
}
impl RmError {
    /// Factory method.
    #[must_use]
    pub fn new(c: ErrorCode, s: String) -> RmError {
        RmError { c, s }
    }
    /// Returns the kind of error that has occured.
    #[must_use]
    pub fn get_code(&self) -> ErrorCode {
        self.c.clone()
    }
    /// Returns a textual description of the error.
    #[must_use]
    pub fn get_description(&self) -> String {
        self.s.clone()
    }
}
