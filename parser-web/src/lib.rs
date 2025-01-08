//! Document parsing API library.
//!
//! This crate provides functionality for parsing various file formats
//! into plain text, exposed through a REST API.

mod config;
mod errors;
mod responses;

pub mod routes;

pub use config::Config;
pub use errors::ApiError;
