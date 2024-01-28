use chrono::{Duration, NaiveDate, NaiveDateTime};
use polars::prelude::{DataFrame, LazyFrame};
use rand::distributions::Uniform;
use rand::{thread_rng, Rng};
use std::error::Error;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::{BufWriter, Write};
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::thread;
// use polars::prelude::*;
use polars_io::prelude::*;

// Generate a random string of length 'len'
pub fn generate_random_string(len: usize) -> String {
    rand::thread_rng()
        .sample_iter(Uniform::new(char::from(97), char::from(122)))
        .take(len)
        .map(char::from)
        .collect()
}

// Generate a random character
pub fn generate_random_char() -> char {
    // choose a random character from the ascii table then convert it
    let num = rand::thread_rng().gen_range(97..=122);

    return char::from_u32(num).unwrap();
}

// Generate a random integer in the range [min, max]
pub fn generate_random_int(min: i32, max: i32) -> i32 {
    thread_rng().gen_range(min..=max)
}
// Generate a random float in the range [min, max)
pub fn generate_random_float(min: f64, max: f64) -> f64 {
    thread_rng().gen_range(min..=max)
}

// Generate a random boolean
pub fn generate_random_bool() -> bool {
    thread_rng().gen_bool(0.5)
}

// Generate a random date in the range [start, end]
pub fn generate_random_date(start: NaiveDate, end: NaiveDate) -> NaiveDate {
    let days = (end - start).num_days();
    start + Duration::days(thread_rng().gen_range(0..=days + 1))
}

// Generate a random datetime in the range [start, end]
pub fn generate_random_datetime(start: NaiveDateTime, end: NaiveDateTime) -> NaiveDateTime {
    let secs = (end - start).num_seconds();
    start + Duration::seconds(thread_rng().gen_range(0..=secs + 1))
}

pub fn generate_headers(name: &String) -> PathBuf {
    let mut file = File::create(name).unwrap();
    let headers = "string_col,char_col,int_col,float_col,bool_col,date_col,datetime_col";
    // write the headers
    writeln!(file, "{}", headers).unwrap();
    return PathBuf::from(name);
}

pub fn generate_csv_file(
    rows: &i32,
    file_name: &String,
    n_threads: &i32,
) -> Result<(), Box<dyn Error>> {
    let rows_per_thread = *rows / *n_threads;
    // create the headers
    let file_path = generate_headers(file_name);
    // create a shared mutable state for the file
    let append_file = OpenOptions::new()
        .write(true)
        .append(true)
        .open(file_path)
        .unwrap();
    let file_writer = Arc::new(Mutex::new(BufWriter::new(append_file)));

    // spawn multiple threads to create the file
    let mut handles = Vec::new();
    for i in 0..(*n_threads) {
        let file_writer_clone = file_writer.clone();
        let start = i * rows_per_thread;
        let end = if i == *(n_threads) - 1 {
            *rows
        } else {
            start + rows_per_thread
        };
        handles.push(thread::spawn(move || {
            for _ in start..end {
                let string_val = generate_random_string(10);
                let char_val = generate_random_char();
                let int_val = generate_random_int(1, 100);
                let float_val = generate_random_float(0.0, 1.0);
                let bool_val = generate_random_bool();
                let date_val = generate_random_date(
                    NaiveDate::from_ymd_opt(2022, 1, 1).unwrap(),
                    NaiveDate::from_ymd_opt(2023, 12, 31).unwrap(),
                );
                let datetime_val = generate_random_datetime(
                    NaiveDateTime::from_timestamp_opt(1640995200, 0).unwrap(),
                    NaiveDateTime::from_timestamp_opt(1643620800, 0).unwrap(),
                );

                let row = format!(
                    "{},{},{},{},{},{},{}\n",
                    string_val, char_val, int_val, float_val, bool_val, date_val, datetime_val
                );
                let mut file_writer = file_writer_clone.lock().unwrap();
                file_writer.write(row.as_bytes()).unwrap();
            }
        }));
    }
    for handle in handles {
        handle.join().unwrap();
    }

    // Flush the file writer to make sure all data is written to disk
    let mut file_writer = file_writer.lock().unwrap();
    file_writer.flush().unwrap();

    Ok(())
}

/// Converts a CSV on disk to Parquet
pub fn convert_csv_to_parquet(
    csv_path: &String,
    parquet_file_name: &String,
) -> Result<(), Box<dyn Error>> {
    let parq_file_name = format!("{}", *parquet_file_name);
    // create the parquet file
    let mut parq_file = File::create(parq_file_name).unwrap();
    let mut df: DataFrame = CsvReader::from_path(csv_path)
        .unwrap()
        .has_header(true)
        .with_try_parse_dates(true)
        .finish()
        .unwrap();

    let _ = ParquetWriter::new(&mut parq_file).finish(&mut df);
    Ok(())
}

pub fn convert_parquet_to_csv(
    parquet_path: &String,
    csv_file_name: &String,
) -> Result<(), Box<dyn Error>> {
    let csv_file_name = format!("{}", *csv_file_name);
    // create the parquet file
    let mut csv_file = File::create(csv_file_name).unwrap();
    let mut df = LazyFrame::scan_parquet(parquet_path, Default::default())?
        .collect()
        .unwrap();
    let _ = CsvWriter::new(&mut csv_file).finish(&mut df);
    Ok(())
}
