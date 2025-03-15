use clap::Parser;
use parser_cli::{parse_files, Cli};

fn main() {
    let cli = Cli::parse();

    match parse_files(&cli.files) {
        Ok(parsed_texts) => {
            // Print each parsed text
            parsed_texts.iter().for_each(|text| println!("{}", text));
        }
        Err(error) => {
            eprintln!("Error parsing files: {:?}", error);
        }
    }
}
