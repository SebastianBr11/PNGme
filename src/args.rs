use clap::{Args, Parser, Subcommand};

#[derive(Debug, Parser)]
pub struct PngMeArgs {
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
    /// Encode a message into a png file
    Encode(EncodeArgs),

    /// Decode a message from a png file
    Decode(DecodeArgs),

    /// Remove message from a png file
    Remove(RemoveArgs),

    /// Print message of a png file
    Print(PrintArgs),
}

#[derive(Debug, Args)]
pub struct EncodeArgs {
    pub file_path: String,
    pub chunk_type: String,
    pub message: String,
    pub output_file: Option<String>,
}

#[derive(Debug, Args)]
pub struct DecodeArgs {
    pub file_path: String,
    pub chunk_type: String,
}

#[derive(Debug, Args)]
pub struct RemoveArgs {
    pub file_path: String,
    pub chunk_type: String,
}

#[derive(Debug, Args)]
pub struct PrintArgs {
    pub file_path: String,
}
