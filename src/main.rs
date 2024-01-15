mod args;
mod fns;

use args::Commands;
use clap::Parser;
use fns::generate_csv_file;

fn main() {
    let cli = args::Cli::parse();

    match cli.command {
        Commands::Create(args) => {
            let rows = args.rows;
            let name = args.name;
            let n_files = args.multiple.unwrap_or(1);
            let num_threads = args.threads.unwrap_or(4);
            let file_type = args.file_type.unwrap_or(String::from("csv"));
            let file_name = format!("{}.{}", name, file_type);

            if n_files > 1 {
                for i in 1..n_files + 1 {
                    let new_file_name = format!("{}_{}.{}", name, i, file_type);
                    if let Ok(()) = generate_csv_file(&rows, &new_file_name, &num_threads) {
                        println!("File {} successfully created", new_file_name);
                    }
                }
            } else {
                if let Ok(()) = generate_csv_file(&rows, &file_name, &num_threads) {
                    println!("File sucessfully created");
                }
            }
        }
    }
}
