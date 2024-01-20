mod args;
mod fns;

use args::Commands;
use clap::Parser;
use log::info;
use std::{fs, path::Path, ffi::OsStr, string::String};

use fns::{convert_csv_to_parquet, generate_csv_file};

fn main() {
    env_logger::init();
    let cli = args::Cli::parse();

    match cli.command {
        Commands::Create(args) => {
            let rows = args.rows;
            let name = args.name;
            let n_files = args.multiple.unwrap_or(1);
            let num_threads = args.threads.unwrap_or(4);
            let file_type = args.file_type.unwrap_or(String::from("csv"));
            let file_name = format!("{}.{}", name, file_type);
            let csv_file_name = format!("{}.csv", name);

            if n_files > 1 {
                for i in 1..n_files + 1 {
                    // let new_file_name = format!("{}_{}.{}", name, i, file_type);
                    let csv_file_path = format!("{}_{}.csv", name, i);
                    if let Ok(()) = generate_csv_file(&rows, &csv_file_path, &num_threads) {
                        if file_type == "csv" {
                            println!("CSV file {} successfully created", csv_file_path);
                        } else if file_type == "parquet" {
                            let parq_file_name = format!("{}_{}.parquet", name, i);
                            convert_csv_to_parquet(&csv_file_path, &parq_file_name);
                            if let Ok(()) = std::fs::remove_file(csv_file_path) {
                                println!(
                                    "Successfully created parquet file {} of {} with {} rows",
                                    i, n_files, rows
                                );
                            }
                        }
                    }
                }
            } else {
                if let Ok(()) = generate_csv_file(&rows, &csv_file_name, &num_threads) {
                    println!("CSV created successfully with {} rows", rows);
                    if file_type == "csv" {
                    } else if file_type == "parquet" {
                        // convert to parquet and remove the csv
                        let parq_file_name = format!("{}.parquet", name);
                        convert_csv_to_parquet(&csv_file_name, &parq_file_name);
                        // delete the csv
                        if let Ok(()) = std::fs::remove_file(csv_file_name) {
                            println!("Successfully created parquet file with {} rows", rows);
                        }
                    }
                }
            }
        },
        // handle the convert command
        Commands::Convert(args) => {
            let source = args.source;
            let file_type = args.file_type;
            let source_str = source.to_owned();

            let extension = Path::new(&source_str)
                .extension()
                .and_then(OsStr::to_str)
                .unwrap();

            let file_name = Path::new(&source_str)
                .file_name()
                .and_then(OsStr::to_str)
                .unwrap();

            let file_name_string = file_name.to_string();

            let file_name_no_extension = Path::new(&source_str)
                .file_stem()
                .unwrap();

            let file_name_no_extension_string = file_name_no_extension.to_str().unwrap();

            match extension {
                "csv" => {
                    // convert the csv to parquet
                    if file_type == "parquet" {
                        let parq_name = format!("{}.parquet", file_name_no_extension_string);
                        convert_csv_to_parquet(&source, &parq_name);
                    }
                },
                "parquet"  => {

                },
                _ => {
                    panic!("Error: Unsupported file type. Options for conversion are csv or parquet")
                }


            }

        }
    }
}
