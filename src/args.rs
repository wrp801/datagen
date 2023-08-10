use clap::Parser; 
#[derive(Parser, Debug)]
pub struct Args {
    #[arg(short = 'r', long = "rows", value_name = "ROWS", help = "Number of rows to generate")]
    pub rows: i32,
    
    #[arg(short = 'n', long = "name", value_name = "NAME", help = "The name of the file to make")]
    pub name: String,

    #[arg(short = 'm', long = "multiple", value_name = "MULTIPLE", required = false, help = "The number of files to be created")]
    pub multiple: Option<i32>,

    #[arg(short = 't', long = "threads", value_name = "THREADS", required = false, help = "The number of threads to use, defaults to 4")]
    pub threads: Option<i32>

}
