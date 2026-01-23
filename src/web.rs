//! Web server functionality.

pub mod errors;
mod routes;

pub use routes::{parse_file, serve_files};
