use clap::Parser;
use parser_core::parse;
use rayon::prelude::*;
use std::{
    fs::{create_dir_all, read, write},
    path::PathBuf,
};

#[derive(Parser)]
#[command(name = "parser")]
#[command(about = "CLI for parsing various document formats", long_about = None)]
struct Cli {
    /// Files to parse
    #[arg(required = true)]
    files: Vec<PathBuf>,

    /// Output directory for the parsed content
    #[arg(short, long)]
    output: Option<PathBuf>,
}

fn main() {
    let cli = Cli::parse();
    let files = cli.files;
    let output = cli.output;

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
            if let Some(output_dir) = output {
                save_to_files(results, output_dir);
            } else {
                // Print to stdout
                for result in results {
                    println!("{}", result);
                }
            }
        }
        Err(e) => eprintln!("Error parsing files: {}", e),
    }
}

fn save_to_files(results: Vec<String>, output_dir: PathBuf) {
    // Create output directory if it doesn't exist
    if !output_dir.exists() {
        if let Err(e) = create_dir_all(&output_dir) {
            eprintln!("Failed to create output directory: {}", e);
            return;
        }
    }

    // Save each result to a file
    for (i, content) in results.iter().enumerate() {
        let file_path = output_dir.join(format!("parsed_{}.txt", i + 1));
        if let Err(e) = write(&file_path, content) {
            eprintln!("Failed to write to {}: {}", file_path.display(), e);
        } else {
            println!("Saved parsed content to {}", file_path.display());
        }
    }
}
