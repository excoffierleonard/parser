//! Routes module for the web server.

mod parse;
mod static_files;

pub use parse::parse_file;
pub use static_files::serve_files;
