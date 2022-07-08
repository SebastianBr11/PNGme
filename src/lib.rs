use args::Command;
use args::PngMeArgs;
use clap::Parser;
use commands::decode::Decode;
use commands::encode::Encode;
use commands::print::Print;
use commands::remove::Remove;

mod args;
mod chunk;
mod chunk_type;
mod commands;
mod png;
pub mod web;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

pub fn run() -> Result<()> {
    let args = PngMeArgs::parse();

    match args.command() {
        Command::Encode(encode_args) => {
            let encode = Encode::new(&encode_args);
            encode.encode()?;
        }
        Command::Decode(decode_args) => {
            let decode = Decode::new(&decode_args);
            let message = decode.decode()?;

            println!("Decoded message found:");
            println!("'{message}'");
        }
        Command::Remove(remove_args) => {
            let remove = Remove::new(&remove_args);
            let removed_message = remove.remove()?;

            println!("Removed message '{removed_message}'");
        }
        Command::Print(print_args) => {
            let print = Print::new(&print_args);
            print.print()?;
        }
    };

    Ok(())
}
