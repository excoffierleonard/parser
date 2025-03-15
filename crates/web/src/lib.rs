//! Document parsing API library.
//!
//! This crate provides functionality for parsing various file formats
//! into plain text, exposed through a REST API.

mod errors;
mod routes;

pub use routes::{parse_file, serve_files};
