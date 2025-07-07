use std::path::PathBuf;

use anyhow::Result;
use clap::{Parser, Subcommand};

use ptcr::util::print_ptcr_file;

#[derive(Parser)]
#[command(author, version, about = "PTCR command line tool")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Print comments with surrounding source code
    Print {
        /// PTCR file to read
        ptcr_file: PathBuf,
        /// Number of context lines
        #[arg(short = 'C', long, default_value_t = 0)]
        context: usize,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Commands::Print { ptcr_file, context } => print_ptcr_file(&ptcr_file, context)?,
    }
    Ok(())
}

