//! Web API functionality.

mod errors;
mod routes;

pub use routes::{parse_file, serve_files};
