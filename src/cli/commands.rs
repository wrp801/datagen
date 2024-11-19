use crate::cli::args::{ConvertArgs, CreateArgs, MultipleArgs};
use crate::cli::fns::{convert_csv_to_parquet, convert_parquet_to_csv, generate_csv_file};
use std::error::Error;
use std::{ffi::OsStr, path::Path, string::String};
use glob::glob;

/// Creates multiple files into either a CSV or Parquet. Each file name will be prefixed with
/// <filename>_<index>
fn create_multiple(
    name: &String,
    rows: &i32,
    n_files: &i32,
    n_threads: &i32,
    file_type: &String,
) -> Result<(), Box<dyn Error>> {
    for i in 1..n_files + 1 {
        let csv_file_path = format!("{}_{}.csv", name, i);
        if let Ok(()) = generate_csv_file(rows, &csv_file_path, n_threads) {
            if file_type == "csv" {
                println!("CSV file {} successfully created", csv_file_path);
            } else if file_type == "parquet" {
                let parq_file_name = format!("{}_{}.parquet", name, i);
                if let Ok(()) = convert_csv_to_parquet(&csv_file_path, &parq_file_name) {
                    println!("Successfully created parquet file {}", parq_file_name);
                    if let Ok(())= std::fs::remove_file(csv_file_path) {
                        println!(
                            "Successfully created parquet file {} of {} with {} rows",
                            i, n_files, rows
                        );
                    }
                }
            }
        }
    }
    Ok(())
}

/// Creates a single CSV or parquet file with the specified number of rows
fn create_single(
    name: &String,
    rows: &i32,
    n_threads: &i32,
    file_type: &String,
) -> Result<(), Box<dyn Error>> {
    let csv_file_name = format!("{}.csv", name);
    if let Ok(()) = generate_csv_file(rows, &csv_file_name, n_threads) {
        println!("CSV created successfully with {} rows", rows);
        if file_type == "csv" {
        } else if file_type == "parquet" {
            // convert to parquet and remove the csv
            let parq_file_name = format!("{}.parquet", name);
            if let Ok(()) = convert_csv_to_parquet(&csv_file_name, &parq_file_name) {
                // delete the csv
                if let Ok(()) = std::fs::remove_file(csv_file_name.clone()) {
                    println!("Successfully created parquet file with {} rows", rows);
                } else {
                    println!("Could not remove {}", csv_file_name.clone());
                }
            } else {
                println!("Error in converting {} to parquet", csv_file_name.clone());
            }
        }
    }
    Ok(())
}

/// Helper function that will either create a single or multiple files based on the provided inputs
pub fn datagen_create(args: &CreateArgs) -> Result<(), Box<dyn Error>> {
    let rows = args.rows;
    let name = args.name.clone();
    let n_files = args.multiple.unwrap_or(1);
    let n_threads = args.threads.unwrap_or(4);
    let file_type = args.file_type.clone().unwrap_or(String::from("csv"));

    if n_files > 1 {
        create_multiple(&name, &rows, &n_files, &n_threads, &file_type)
    } else {
        create_single(&name, &rows, &n_threads, &file_type)
    }
}

fn convert_file(
    extension: &str,
    file_type: &String,
    file_name_no_extension_string: &str,
    source: &String,
) -> Result<(), Box<dyn Error>> {
    match extension {
        "csv" => {
            // convert the csv to parquet
            if file_type == "parquet" {
                let parq_name = format!("{}.parquet", file_name_no_extension_string);
                if let Ok(()) = convert_csv_to_parquet(source, &parq_name) {
                    println!("Successfully converted {} to {}", source, parq_name);
                }
            }
        }
        "parquet" => {
            // convert the parquet to csv
            if file_type == "csv" {
                let csv_name = format!("{}.csv", file_name_no_extension_string);
                if let Ok(()) = convert_parquet_to_csv(source, &csv_name) {
                    println!("Successfully converted {} to {}", source, csv_name);
                }
            }
        }
        _ => {
            panic!("Error: Unsupported file type. Options for conversion are csv or parquet")
        }
    }
    Ok(())
}

pub fn datagen_convert(args: &ConvertArgs) -> Result<(), Box<dyn Error>> {
    let source = args.source.clone().unwrap();
    let file_type = args.file_type.clone().unwrap();
    let source_str = source.to_owned();

    let extension = Path::new(&source_str)
        .extension()
        .and_then(OsStr::to_str)
        .unwrap();

    let file_name_no_extension = Path::new(&source_str).file_stem().unwrap();

    let file_name_no_extension_string = file_name_no_extension.to_str().unwrap();

    convert_file(
        extension,
        &file_type,
        file_name_no_extension_string,
        &source,
    )?;

    Ok(())
}


/// converts multiple files
pub fn convert_multiple(args: &MultipleArgs) -> Result<(), Box<dyn Error>> {
    let pattern = args.pattern.clone();
    let file_type = args.file_type.clone();
    let source_str = pattern.to_owned();
    let file_type_str = file_type.as_str();

    for entry in glob(&(pattern.to_owned())).expect("Could not read glob pattern") {
        match entry {
            // Ok(path) => println!("{:?}", path.display()),
            Ok(path) => {
                let file_name = "";
                let extension = Path::new(&source_str)
                    .extension()
                    .and_then(OsStr::to_str)
                    .unwrap();

                let file_name_no_extension = Path::new(&source_str)
                    .file_stem()
                    .unwrap();
                match file_type_str {
                    "csv" => {
                        println!("Entry is {:?}", path.display());
                    }
                    "parquet" => {

                    }
                    _ => {
                        println!("File type not supported")

                    }
                }

            },
            Err(e) => println!("{:?}", e),
        }
    }

    Ok(())
}
