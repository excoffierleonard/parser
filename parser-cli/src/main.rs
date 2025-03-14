use clap::Parser;
use parser_core::parse;
use rayon::prelude::*;
use std::{
    fs::read,
    path::PathBuf,
};

#[derive(Parser)]
#[command(name = "parser")]
#[command(about = "CLI for parsing various document formats", long_about = None)]
struct Cli {
    /// Files to parse
    #[arg(required = true)]
    files: Vec<PathBuf>,
}

fn main() {
    let cli = Cli::parse();
    let files = cli.files;

    // Read all files into memory and collect their data as slices
    let file_data: Vec<_> = files.iter().filter_map(|path| read(path).ok()).collect();

    // Create a slice of slices for processing
    let file_slices: Vec<&[u8]> = file_data.iter().map(|d| d.as_slice()).collect();

    match file_slices
        .par_iter()
        .map(|d| parse(d))
        .collect::<Result<Vec<_>, _>>()
    {
        Ok(results) => {
            // Print to stdout
            for result in results {
                println!("{}", result);
            }
        }
        Err(e) => eprintln!("Error parsing files: {}", e),
    }
}
