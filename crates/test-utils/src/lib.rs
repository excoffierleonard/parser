//! Common test utilities for the parser project
//!
//! This crate provides shared functionality for testing across all parser crates.

use std::{fs, path::PathBuf};

/// Returns the path to the centralized test inputs directory
pub fn test_inputs_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("assets")
}

/// Returns the path to a specific test file in the inputs directory
pub fn test_file_path(filename: &str) -> PathBuf {
    test_inputs_dir().join(filename)
}

/// Reads a test file and returns its contents as bytes
pub fn read_test_file(filename: &str) -> Vec<u8> {
    fs::read(test_file_path(filename)).unwrap_or_else(|e| {
        panic!("Failed to read test file {}: {}", filename, e);
    })
}
