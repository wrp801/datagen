use clap::Parser; 
use chrono::{NaiveDate, NaiveDateTime};
use std::fs::{File, OpenOptions};
use std::io::{Write, BufWriter};
use std::sync::{Arc, Mutex};
use std::thread;

mod fns;
mod args;

fn main() {
    let args = args::Args::parse();
    let rows = *(&args.rows); //convert to i32 
    let name = &args.name; 

    let num_threads = match &args.threads {
        None => 4,
        Some(x) => *(Some(x).unwrap())
    };

    // concurrency test
    // let num_threads = 4;
    let rows_per_thread = rows/num_threads;
    // create the headers
    let file_path = fns::generate_headers(name);
    // create a shared mutable state for the file
    let append_file = OpenOptions::new().write(true).append(true).open(file_path).unwrap();
    let file_writer = Arc::new(Mutex::new(BufWriter::new(append_file)));

    // spawn multiple threads to create the file
    let mut handles = Vec::new();
    for i in 0..num_threads {
        let file_writer_clone = file_writer.clone();
        let start = i * rows_per_thread;
        let end = if i == num_threads - 1 {rows} else {start + rows_per_thread};
        handles.push(thread::spawn(move || {
            for _ in start..end {
                let string_val = fns::generate_random_string(10);
                let char_val = fns::generate_random_char();
                let int_val = fns::generate_random_int(1, 100);
                let float_val = fns::generate_random_float(0.0, 1.0);
                let bool_val = fns::generate_random_bool();
                let date_val = fns::generate_random_date(NaiveDate::from_ymd(2022, 1, 1), NaiveDate::from_ymd(2023, 12, 31));
                let datetime_val = fns::generate_random_datetime(NaiveDateTime::from_timestamp(1640995200, 0), NaiveDateTime::from_timestamp(1643620800, 0));
                
                let row = format!("{},{},{},{},{},{},{}\n", string_val, char_val, int_val, float_val, bool_val, date_val, datetime_val);
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
}
