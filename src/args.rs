use clap::{Args, Parser, Subcommand};

#[derive(Debug, Parser)]
/// A command line program that lets you hide secret messages in PNG files.
pub struct PngMeArgs {
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
    Encode(EncodeArgs),

    /// Decode a message stored in a PNG file
    Decode(DecodeArgs),

    /// Remove message from a png file
    Remove(RemoveArgs),

    /// Print a list of PNG chunks
    Print(PrintArgs),
}

#[derive(Debug, Args)]
pub struct EncodeArgs {
    pub file_path: String,
    pub chunk_type: String,
    pub message: String,
    #[clap(required_if_eq("web", "true"))]
    pub output_file: Option<String>,
    #[clap(from_global)]
    pub web: bool,
}

#[derive(Debug, Args)]
pub struct DecodeArgs {
    pub file_path: String,
    pub chunk_type: String,
    #[clap(from_global)]
    pub web: bool,
}

#[derive(Debug, Args)]
pub struct RemoveArgs {
    pub file_path: String,
    pub chunk_type: String,
    #[clap(required_if_eq("web", "true"))]
    pub output_file: Option<String>,
    #[clap(from_global)]
    pub web: bool,
}

#[derive(Debug, Args)]
pub struct PrintArgs {
    pub file_path: String,
}
