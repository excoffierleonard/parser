//! Routes module for the API.
//!
//! This module contains all route handlers for the application,
//! organizing them by functionality.

mod hello;
mod parse;
mod static_files;

pub use hello::greet;
pub use parse::parse_file;
pub use static_files::serve_files;
