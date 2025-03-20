use clap::Parser;
use parser_core::{parse, ParserError};
use rayon::prelude::*;
use std::{fs::read, io::Error, path::PathBuf};

/// CLI arguments parser
#[derive(Parser)]
#[command(about = "CLI for parsing various document formats", long_about = None)]
pub struct Cli {
    /// Files to parse
    #[arg(required = true)]
    pub files: Vec<PathBuf>,
}

/// Parses files in parallel and returns a Result containing either all parsed texts or the first error
pub fn parse_files(paths: &[PathBuf]) -> Result<Vec<String>, ParserError> {
    // Read files into memory
    let files = paths
        .iter()
        .map(read)
        .collect::<Result<Vec<Vec<u8>>, Error>>()?;

    // Process files in parallel
    files
        .par_iter()
        .map(|data| parse(data))
        .collect::<Result<Vec<String>, ParserError>>()
}
