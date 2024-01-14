use clap::Parser;
use clap::{Args, Subcommand};

#[derive(Debug, Parser)]
#[clap(name = "datagen", version)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Create a randomly generated csv file
    Create(CreateArgs),
    // Convert(Convert),
}

#[derive(Debug,Parser)]
pub struct CreateArgs {
    /// Number of rows to generate
    #[arg(short = 'r', long = "rows")]
    pub rows: i32,

    /// The name of the file to make. Defaults to sample
    #[arg(short = 'f', long = "filename")]
    pub name: String,

    /// The number of files to be created
    #[arg(short = 'm', long = "multiple", required = false)]
    pub multiple: Option<i32>,

    /// The number of threads to use, defaults to 4
    #[arg(short = 't', long = "threads", required = false)]
    pub threads: Option<i32>,

    /// The file type to be written, defaults to csv
    #[arg(short = 'y', long = "file-type", required = false)]
    pub file_type: Option<String>,
}

