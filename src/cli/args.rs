use clap::Parser;
use clap::Subcommand;

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
    Convert(ConvertArgs),
}

#[derive(Debug, Parser)]
pub struct CreateArgs {

    /// The name of the file to make. Defaults to sample
    pub name: String,

    /// Number of rows to generate
    #[clap(short = 'r', long = "rows")]
    pub rows: i32,


    /// The number of files to be created
    #[clap(short = 'm', long = "multiple", required = false)]
    pub multiple: Option<i32>,

    /// The number of threads to use, defaults to 4
    #[clap(short = 't', long = "threads", required = false)]
    pub threads: Option<i32>,

    /// The file type to be written (options are xlsx, csv, and parquet), defaults to csv
    #[clap(short = 'y', long = "file-type", required = false)]
    pub file_type: Option<String>,
}

#[derive(Debug, Parser)]
pub struct ConvertArgs {
    /// Command designating that datagen should convert multiple files
    #[clap(subcommand)]
    pub multiple: Option<MultipleCommand>,

    /// The name of the source file to convert
    #[clap(short = 's', long = "source")]
    pub source: Option<String>,

    /// The type of file to convert to
    #[clap(short = 'y', long = "file-type")]
    pub file_type: Option<String>,

}

/// Command designating that datagen should convert multiple files
#[derive(Debug, Subcommand)]
pub enum MultipleCommand {
    Multiple(MultipleArgs),
}

/// Args for Multiple
#[derive(Debug, Parser)]
pub struct MultipleArgs {
    /// The glob pattern of the files to match against
    #[clap(short = 'p', long = "pattern")]
    pub pattern: String,

    /// The file type to convert to
    #[clap(short = 'y', long = "file-type")]
    pub file_type: String,
}
