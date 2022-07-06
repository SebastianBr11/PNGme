use args::Command;
use args::PngMeArgs;
use clap::Parser;

mod args;
mod chunk;
mod chunk_type;
// mod commands;
mod png;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

fn main() -> Result<()> {
    let args = PngMeArgs::parse();

    match args.command() {
        Command::Encode(encode_args) => println!("encoding with {encode_args:?}"),
        Command::Decode(decode_args) => println!("encoding with {decode_args:?}"),
        Command::Remove(remove_args) => println!("encoding with {remove_args:?}"),
        Command::Print(print_args) => println!("encoding with {print_args:?}"),
    }

    Ok(())
}
