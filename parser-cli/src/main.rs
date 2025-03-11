use clap::{Parser, Subcommand};
use parser_core::InputFiles;
use std::{
    fs::{create_dir_all, read, write},
    path::PathBuf,
};

#[derive(Parser)]
#[command(name = "parser")]
#[command(about = "CLI for parsing various document formats", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Parse files and extract their text content
    Parse {
        /// Files to parse
        #[arg(required = true)]
        files: Vec<PathBuf>,

        /// Output directory for the parsed content
        #[arg(short, long)]
        output: Option<PathBuf>,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Parse { files, output } => {
            // Read all files into memory and collect their data
            let file_data = files
                .iter()
                .filter_map(|path| {
                    read(path)
                        .ok()
                        .map(|bytes| {
                            // Only use to_string() if path contains non-UTF8 characters
                            let filename = match path.to_str() {
                                Some(s) => s.to_string(),
                                None => path.to_string_lossy().to_string(),
                            };
                            (bytes, filename)
                        })
                })
                .collect();

            let input_files = InputFiles::with_filenames(file_data);
            match input_files.parse() {
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
