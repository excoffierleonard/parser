//! Error handling for the parser library.
//!
//! This module defines the error types used throughout the library and implements
//! conversions from common external error types to our internal error type.

/// Custom error type for the parser library.
///
/// Represents different categories of errors that can occur during parsing operations,
/// including I/O errors, parsing errors, and format validation errors.
#[derive(Debug)]
pub enum ParserError {
    /// An error occurred while reading or writing a file.
    ///
    /// This includes file system errors, issues with file permissions, or
    /// problems with temporary file creation.
    IoError(String),

    /// An error occurred while parsing the content of a file.
    ///
    /// This includes syntax errors, encoding problems, or issues with
    /// the internal structure of documents.
    ParseError(String),

    /// The file has an invalid or unsupported format.
    ///
    /// This occurs when the file type cannot be recognized or is not
    /// supported by the parser library.
    InvalidFormat(String),
}

impl std::fmt::Display for ParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParserError::IoError(msg) => write!(f, "IO error: {msg}"),
            ParserError::ParseError(msg) => write!(f, "Parse error: {msg}"),
            ParserError::InvalidFormat(msg) => write!(f, "Invalid format: {msg}"),
        }
    }
}

/// Implements the `std::error::Error` trait for `ParserError` to allow it to be used
/// with the ? operator and to be boxed as a dyn Error.
impl std::error::Error for ParserError {}

macro_rules! impl_from_error {
    ($type:ty, $variant:expr) => {
        impl From<$type> for ParserError {
            fn from(err: $type) -> Self {
                $variant(err.to_string())
            }
        }
    };
}

// IO errors
impl_from_error!(std::io::Error, ParserError::IoError);
impl_from_error!(tesseract::InitializeError, ParserError::IoError);
impl_from_error!(tesseract::SetImageError, ParserError::IoError);
impl_from_error!(
    tesseract::plumbing::TessBaseApiGetUtf8TextError,
    ParserError::IoError
);

// Parse errors
impl_from_error!(pdf_extract::OutputError, ParserError::ParseError);
impl_from_error!(docx_rs::ReaderError, ParserError::ParseError);
impl_from_error!(std::string::FromUtf8Error, ParserError::ParseError);
impl_from_error!(std::str::Utf8Error, ParserError::ParseError);
impl_from_error!(zip::result::ZipError, ParserError::ParseError);
impl_from_error!(regex::Error, ParserError::ParseError);
impl_from_error!(std::process::ExitStatus, ParserError::ParseError);
impl_from_error!(calamine::XlsxError, ParserError::ParseError);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        // Test each error variant's Display implementation
        let io_err = ParserError::IoError("failed to read file".to_string());
        let parse_err = ParserError::ParseError("failed to parse content".to_string());
        let format_err = ParserError::InvalidFormat("invalid file format".to_string());

        assert_eq!(io_err.to_string(), "IO error: failed to read file");
        assert_eq!(
            parse_err.to_string(),
            "Parse error: failed to parse content"
        );
        assert_eq!(
            format_err.to_string(),
            "Invalid format: invalid file format"
        );
    }
}
