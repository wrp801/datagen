use clap::Parser; 
use chrono::{Duration, NaiveDate, NaiveDateTime};
use rand::distributions::{Distribution, Standard, Alphanumeric, Uniform};
use rand::{thread_rng, Rng};
use std::error::Error;
use std::fs::File;
use std::io::{self, Write};
use std::str::FromStr;
use std::thread;

#[derive(Parser, Debug)]
struct Args {
    #[arg(short = 'r', long = "rows", value_name = "ROWS")]
    rows: i32,
    
    #[arg(short = 'n', long = "name", value_name = "NAME")]
    name: String
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

    // let mut file = File::create("data.csv")?;
    let mut file = File::create(&filename)?;
    writeln!(
        file,
        "string_col,char_col,int_col,float_col,bool_col,date_col,datetime_col"
    )?;

    thread::spawn(|| {
        for _ in 0..n {
            let string_val = generate_random_string(10);
            let char_val = generate_random_char();
            let int_val = generate_random_int(1, 100);
            let float_val = generate_random_float(0.0, 1.0);
            let bool_val = generate_random_bool();
            let date_val = generate_random_date(start_date, end_date);
            let datetime_val = generate_random_datetime(start_datetime, end_datetime);

            writeln!(
                file,
                "{},{},{},{},{},{},{}",
                string_val,
                char_val,
                int_val,
                float_val,
                bool_val,
                date_val,
                datetime_val
            );
        }

    });

    for _ in 0..n {
        let string_val = generate_random_string(10);
        let char_val = generate_random_char();
        let int_val = generate_random_int(1, 100);
        let float_val = generate_random_float(0.0, 1.0);
        let bool_val = generate_random_bool();
        let date_val = generate_random_date(start_date, end_date);
        let datetime_val = generate_random_datetime(start_datetime, end_datetime);

        writeln!(
            file,
            "{},{},{},{},{},{},{}",
            string_val,
            char_val,
            int_val,
            float_val,
            bool_val,
            date_val,
            datetime_val
        )?;
    }

    Ok(())
}


fn main() {
    let args = Args::parse();
    let rows = *(&args.rows); //convert to i32 
    let name = &args.name; 

    let file_name = String::from_str("mytest").unwrap();
    generate_csv_file(rows, name.to_string());
}
