use clap::Parser;
use parser_core::parse;
use rayon::prelude::*;
use std::{fs::read, path::PathBuf};

/// CLI arguments parser
#[derive(Parser)]
#[command(about = "CLI for parsing various document formats", long_about = None)]
pub struct Cli {
    /// Files to parse
    #[arg(required = true)]
    pub files: Vec<PathBuf>,
}

/// Parses files in parallel and returns a Result containing either all parsed texts or the first error
pub fn parse_files(files: &[PathBuf]) -> Result<Vec<String>, parser_core::ParserError> {
    let results: Vec<_> = files
        .par_iter()
        .filter_map(|path| read(path).ok().map(|data| parse(&data)))
        .collect();

    // Check if all results are Ok
    if results.iter().all(|r| r.is_ok()) {
        Ok(results.into_iter().filter_map(Result::ok).collect())
    } else {
        // Return the first error
        Err(results.into_iter().find_map(|r| r.err()).unwrap())
    }
}
