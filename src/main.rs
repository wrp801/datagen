use crate::cli::args;
use crate::cli::commands::{datagen_convert, datagen_create, convert_multiple};
use args::Commands;
use clap::Parser;
use cli::args::MultipleCommand;

pub mod cli;

fn main() {
    let cli = args::Cli::parse();

    match cli.command {
        Commands::Create(args) => {
            let _ = datagen_create(&args);
        }
        // handle the convert command
        Commands::Convert(convert_args) => match convert_args.multiple {
            Some(MultipleCommand::Multiple(multiple_args)) => {
                // handle multiple cases
                convert_multiple(&multiple_args);

            }
            None => {
                let _ = datagen_convert(&convert_args);
            }
        }
    }
}
