use clap::Parser; 
use chrono::{Duration, NaiveDate, NaiveDateTime};
use rand::distributions::{Distribution, Uniform};
use rand::{thread_rng, Rng};
use std::error::Error;
use std::fs::{File, OpenOptions};
use std::io::{Write, BufWriter};
use std::path::PathBuf;

use std::sync::{Arc, Mutex};
use std::thread;

#[derive(Parser, Debug)]
struct Args {
    #[arg(short = 'r', long = "rows", value_name = "ROWS", help = "Number of rows to generate")]
    rows: i32,
    
    #[arg(short = 'n', long = "name", value_name = "NAME", help = "The name of the file to make")]
    name: String,

    #[arg(short = 'm', long = "multiple", value_name = "MULTIPLE", required = false, help = "The number of files to be created")]
    multiple: Option<i32>,

    #[arg(short = 't', long = "threads", value_name = "THREADS", required = false, help = "The number of threads to use, defaults to 4")]
    threads: Option<i32>

}

// Generate a random string of length 'len'
fn generate_random_string(len: usize) -> String {
    rand::thread_rng()
        .sample_iter(Uniform::new(char::from(97), char::from(122)))
        .take(len)
        .map(char::from)
        .collect()
}

// Generate a random character
fn generate_random_char() -> char {
    // choose a random character from the ascii table then convert it
   let num =  rand::thread_rng()
        .gen_range(97..=122);

   return char::from_u32(num).unwrap()
}

// Generate a random integer in the range [min, max]
fn generate_random_int(min: i32, max: i32) -> i32 {
    thread_rng().gen_range(min..=max)
}
// Generate a random float in the range [min, max)
fn generate_random_float(min: f64, max: f64) -> f64 {
    thread_rng().gen_range(min..=max)
}

// Generate a random boolean
fn generate_random_bool() -> bool {
    thread_rng().gen_bool(0.5)
}

// Generate a random date in the range [start, end]
fn generate_random_date(start: NaiveDate, end: NaiveDate) -> NaiveDate {
    let days = (end - start).num_days();
    start + Duration::days(thread_rng().gen_range(0..=days + 1))
}

// Generate a random datetime in the range [start, end]
fn generate_random_datetime(start: NaiveDateTime, end: NaiveDateTime) -> NaiveDateTime {
    let secs = (end - start).num_seconds();
    start + Duration::seconds(thread_rng().gen_range(0..=secs + 1))
}
// Generate 'n' rows of random data and write to a CSV file
fn generate_csv_file(n: i32, file_name:String) -> Result<(), Box<dyn Error>> {
    let start_date = NaiveDate::from_ymd(2022, 1, 1);
    let end_date = NaiveDate::from_ymd(2023, 12, 31);
    let start_datetime = NaiveDateTime::from_timestamp(1640995200, 0);
    let end_datetime = NaiveDateTime::from_timestamp(1643620800, 0);
    let filename = format!("{}.csv", file_name);

    // let mut file = File::create(&filename)?;
    let mut file_writer = BufWriter::new(File::create(&file_name).unwrap());

    for _ in 0..n {
        let string_val = generate_random_string(10);
        let char_val = generate_random_char();
        let int_val = generate_random_int(1, 100);
        let float_val = generate_random_float(0.0, 1.0);
        let bool_val = generate_random_bool();
        let date_val = generate_random_date(start_date, end_date);
        let datetime_val = generate_random_datetime(start_datetime, end_datetime);

        // writeln!(
        //     file,
        //     "{},{},{},{},{},{},{}",
        //     string_val,
        //     char_val,
        //     int_val,
        //     float_val,
        //     bool_val,
        //     date_val,
        //     datetime_val
        // );

        let row = format!("{},{},{},{},{},{},{}\n", string_val, char_val, int_val, float_val, bool_val, date_val, datetime_val);
        file_writer.write(row.as_bytes()).unwrap();
    }
    file_writer.flush().unwrap();

    Ok(())
}

fn generate_headers(name:&String) -> PathBuf {
    let mut file = File::create(name).unwrap();
    // let file_writer = BufWriter::new(File::create(name).unwrap());
    let headers = "string_col,char_col,int_col,float_col,bool_col,date_col,datetime_col";
    // write the headers
    writeln!(file, "{}", headers).unwrap();
    return PathBuf::from(name)
}


fn main() {
    let args = Args::parse();
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
    let file_path = generate_headers(name);
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
                let string_val = generate_random_string(10);
                let char_val = generate_random_char();
                let int_val = generate_random_int(1, 100);
                let float_val = generate_random_float(0.0, 1.0);
                let bool_val = generate_random_bool();
                let date_val = generate_random_date(NaiveDate::from_ymd(2022, 1, 1), NaiveDate::from_ymd(2023, 12, 31));
                let datetime_val = generate_random_datetime(NaiveDateTime::from_timestamp(1640995200, 0), NaiveDateTime::from_timestamp(1643620800, 0));
                
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
