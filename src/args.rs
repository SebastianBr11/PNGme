use clap::{Args, Parser, Subcommand};

#[derive(Debug, Parser)]
/// A command line program that lets you hide secret messages in PNG files.
pub struct PngMeArgs {
    /// Get the png file from a url instead of a local path
    #[clap(short, long, global(true))]
    pub web: bool,
    #[clap(subcommand)]
    command: Command,
}

impl PngMeArgs {
    pub fn command(&self) -> &Command {
        &self.command
    }
}

#[derive(Debug, Subcommand)]
pub enum Command {
    /// Encode a message into a PNG file
    #[clap(arg_required_else_help(true))]
    Encode(EncodeArgs),

    /// Decode a message stored in a PNG file
    #[clap(arg_required_else_help(true))]
    Decode(DecodeArgs),

    /// Remove message from a png file
    #[clap(arg_required_else_help(true))]
    Remove(RemoveArgs),

    /// Print a list of PNG chunks
    #[clap(arg_required_else_help(true))]
    Print(PrintArgs),
}

#[derive(Debug, Args)]
pub struct EncodeArgs {
    /// Path to the resource
    pub file_path: String,

    /// Chunk type consisting of 4 charactes
    pub chunk_type: String,

    /// Message to encode
    pub message: String,

    /// File name for the output file
    #[clap(required_if_eq("web", "true"))]
    pub output_file: Option<String>,

    #[clap(from_global)]
    pub web: bool,
}

#[derive(Debug, Args)]
pub struct DecodeArgs {
    /// Path to the resource
    pub file_path: String,

    /// Chunk type consisting of 4 characters
    pub chunk_type: String,

    #[clap(from_global)]
    pub web: bool,
}

#[derive(Debug, Args)]
pub struct RemoveArgs {
    /// Path to the resource
    pub file_path: String,

    /// Chunk type consisting of 4 characters
    pub chunk_type: String,

    /// File name for the output file
    #[clap(required_if_eq("web", "true"))]
    pub output_file: Option<String>,

    #[clap(from_global)]
    pub web: bool,
}

#[derive(Debug, Args)]
pub struct PrintArgs {
    /// Path to the resource
    pub file_path: String,

    #[clap(from_global)]
    pub web: bool,
}
