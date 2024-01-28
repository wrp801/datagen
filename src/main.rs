use crate::cli::args;
use crate::cli::commands::{datagen_convert, datagen_create};
use args::Commands;
use clap::Parser;

pub mod cli;

fn main() {
    let cli = args::Cli::parse();

    match cli.command {
        Commands::Create(args) => {
            let _ = datagen_create(&args);
        }
        // handle the convert command
        Commands::Convert(args) => {
            let _ = datagen_convert(&args);
        }
    }
}
