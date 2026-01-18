use clap::{Parser, Subcommand};
use compressor::{compress, decompress};
use std::fs::File;
use std::io::{self, BufReader, BufWriter};
use std::path::PathBuf; // import your lib functions

/// Simple file compressor CLI
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Compress a file
    #[command(alias = "c")]
    Compress {
        /// Input file path
        #[arg(short, long, value_name = "FILE")]
        input: PathBuf,

        /// Output file path
        #[arg(short, long, value_name = "FILE")]
        output: PathBuf,
    },
    /// Decompress a file
    #[command(alias = "d")]
    Decompress {
        /// Input file path
        #[arg(short, long, value_name = "FILE")]
        input: PathBuf,

        /// Output file path
        #[arg(short, long, value_name = "FILE")]
        output: PathBuf,
    },
}

fn main() -> io::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Compress { input, output } => {
            let input_file = File::open(&input)?;
            let mut reader = BufReader::new(input_file);

            let output_file = File::create(&output)?;
            let mut writer = BufWriter::new(output_file);

            compress(&mut reader, &mut writer)?;
            println!("Compression finished successfully!");
        }
        Commands::Decompress { input, output } => {
            let input_file = File::open(&input)?;
            let mut reader = BufReader::new(input_file);

            let output_file = File::create(&output)?;
            let mut writer = BufWriter::new(output_file);

            decompress(&mut reader, &mut writer)?;
            println!("Decompression finished successfully!");
        }
    }

    Ok(())
}
