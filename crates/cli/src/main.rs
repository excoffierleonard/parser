use clap::Parser;
use parser_core::parse;
use rayon::prelude::*;
use std::{fs::read, path::PathBuf};

#[derive(Parser)]
#[command(about = "CLI for parsing various document formats", long_about = None)]
struct Cli {
    /// Files to parse
    #[arg(required = true)]
    files: Vec<PathBuf>,
}

fn main() {
    let cli = Cli::parse();

    match cli
        .files
        .par_iter()
        .filter_map(|path| read(path).ok().map(|data| parse(&data)))
        .collect::<Vec<_>>()
    {
        results if results.iter().all(|r| r.is_ok()) => {
            results
                .into_iter()
                .filter_map(Result::ok)
                .for_each(|text| println!("{}", text));
        }
        results => eprintln!(
            "Error parsing files: {:?}",
            results.into_iter().find_map(|r| r.err()).unwrap()
        ),
    }
}
