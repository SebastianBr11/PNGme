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
            if encode_args.web {
                encode.encode_web_file()?;
            } else {
                encode.encode_local_file()?;
            }
        }
        Command::Decode(decode_args) => {
            let decode = Decode::new(&decode_args);
            let message: String;

            if decode_args.web {
                message = decode.decode_web_file()?;
            } else {
                message = decode.decode_local_file()?;
            }

            println!("Decoded message found:");
            println!("'{message}'");
        }
        Command::Remove(remove_args) => {
            let remove = Remove::new(&remove_args);
            let removed_message;

            if remove_args.web {
                removed_message = remove.remove_web_encoding()?;
            } else {
                removed_message = remove.remove_local_encoding()?;
            }

            println!("Removed message '{removed_message}'");
        }
        Command::Print(print_args) => {
            let print = Print::new(&print_args);

            if print_args.web {
                print.print_web_chunks()?;
            } else {
                print.print_local_chunks()?;
            }
        }
    };

    Ok(())
}
