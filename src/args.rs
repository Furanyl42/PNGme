use clap::{Args, Parser, Subcommand};
use std::path::PathBuf;

use crate::chunk_type::ChunkType;

#[derive(Parser)]
#[command(version, about, long_about = None)]
/// CLI tool to encode and decode secret messages in PNG files.
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Encode a secret message into a PNG file
    Encode(EncodeArgs),
    /// Decode a secret message from a PNG file
    Decode(DecodeArgs),
    /// Remove a secret chunk from a PNG file
    Remove(RemoveArgs),
    /// Print all chunks in a PNG file
    Print(PrintArgs),
}

#[derive(Args, Debug)]
pub struct EncodeArgs {
    /// Path to the input PNG file
    pub file_path: PathBuf,
    /// 4-character chunk type (e.g., "ruSt")
    pub chunk_type: ChunkType,
    /// Secret message to hide
    pub message: String,
    /// Optional output file path (if omitted, overwrites input file)
    pub output_file: Option<PathBuf>,
}

#[derive(Args, Debug)]
pub struct DecodeArgs {
    /// Path to the PNG file
    pub file_path: PathBuf,
    /// 4-character chunk type containing the message
    pub chunk_type: ChunkType,
}

#[derive(Args, Debug)]
pub struct RemoveArgs {
    /// Path to the PNG file
    pub file_path: PathBuf,
    /// 4-character chunk type to remove
    pub chunk_type: ChunkType,
}

#[derive(Args, Debug)]
pub struct PrintArgs {
    /// Path to the PNG file
    pub file_path: PathBuf,
}
