//! Document parsing library.
//!
//! This crate provides functionality for parsing various file formats

mod errors;
mod parsers;

pub use errors::ParserError;
pub use parsers::{determine_mime_type, parse_any, InputFiles};
