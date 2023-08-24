use clap::Parser; 
use clap::{Subcommand, Args};

pub struct Cli {
    #[command(subcommand)]
    command: Option<Commands>
}

// #[derive(Parser, Debug)]
// pub struct Args {
//     #[arg(short = 'r', long = "rows", value_name = "ROWS", help = "Number of rows to generate")]
//     pub rows: i32,
    
//     #[arg(short = 'n', long = "name", value_name = "NAME", help = "The name of the file to make")]
//     pub name: String,

//     #[arg(short = 'm', long = "multiple", value_name = "MULTIPLE", required = false, help = "The number of files to be created")]
//     pub multiple: Option<i32>,

//     #[arg(short = 't', long = "threads", value_name = "THREADS", required = false, help = "The number of threads to use, defaults to 4")]
//     pub threads: Option<i32>
// }


#[derive(Parser, Debug, Subcommand)]
pub enum Commands {
    /// Create a randomly generated csv file
    Create(Create),
    /// Convert a file from one type to another
    Convert(Convert),
}

#[derive(Args)]
pub struct Create {
    #[arg(short = 'r', long = "rows", value_name = "ROWS", help = "Number of rows to generate")]
    pub rows: i32,
    
    #[arg(short = 'f', long = "filename", value_name = "FILENAME", help = "The name of the file to make")]
    pub name: Option<String>,

    #[arg(short = 'm', long = "multiple", value_name = "MULTIPLE", required = false, help = "The number of files to be created")]
    pub multiple: Option<i32>,

    #[arg(short = 't', long = "threads", value_name = "THREADS", required = false, default_value = 4 ,help = "The number of threads to use, defaults to 4")]
    pub threads: Option<i32>,

    #[arg(short = 'y', long = "file-type", value_name = "FILETYPE", required = false, default_value = ".csv", help = "The file type to be written, defaults to csv")]
    pub file_type: String
}

#[derive(Args)]
pub struct Convert {
    #[arg(short = 'f', long = "filename", value_name = "FILENAME", help = "The name of the file to make")]
    pub name: Option<String>,
}


