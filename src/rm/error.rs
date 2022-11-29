use super::error_code::ErrorCode;

/// Represents the possible errors that can occur in a `ResourceManager`.
#[derive(Debug)]
pub struct Error {
    c: ErrorCode,
    s: String,
}
impl Error {
    /// Factory method.
    #[must_use]
    pub fn new(c: ErrorCode, s: String) -> Error {
        Error { c, s }
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
