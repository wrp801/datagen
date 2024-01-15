mod args;
mod fns;

use args::Commands;
use clap::Parser;
use fns::generate_file;

fn main() {
    let cli = args::Cli::parse();

    match cli.command {
        Commands::Create(args) => {
            let rows = args.rows;
            let name = args.name;
            let n_files = args.multiple.unwrap_or(1);
            let num_threads = args.threads.unwrap_or(4);
            let file_type = args.file_type.unwrap_or(String::from("csv"));
            // let res = generate_file(rows, name, num_threads);

            if let Ok(()) = generate_file(rows, name, num_threads) {
                println!("File sucessfully created");
            }
        }
    }
}
