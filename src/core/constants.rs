//! Constants used throughout the parser library.
//!
//! Contains MIME type constants for various document formats supported by the parser.

/// MIME type for PDF documents
pub const APPLICATION_PDF: &str = "application/pdf";

/// MIME type for DOCX (Microsoft Word) documents
pub const APPLICATION_DOCX: &str =
    "application/vnd.openxmlformats-officedocument.wordprocessingml.document";

/// MIME type for XLSX (Microsoft Excel) spreadsheets
pub const APPLICATION_XLSX: &str =
    "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet";

/// MIME type for PPTX (Microsoft `PowerPoint`) presentations
pub const APPLICATION_PPTX: &str =
    "application/vnd.openxmlformats-officedocument.presentationml.presentation";
