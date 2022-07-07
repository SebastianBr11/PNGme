use std::fmt::Display;
pub use std::{fs, str::FromStr};

use crate::args::PrintArgs;
pub use crate::{chunk::Chunk, chunk_type::ChunkType, png::Png, web, Result};

pub mod decode;
pub mod encode;
pub mod remove;

pub fn print_chunks(args: &PrintArgs) -> Result<()> {
    let file = &fs::read(&args.file_path)?;
    let png = Png::try_from(&file[..])?;

    println!("{png}");

    Ok(())
}

#[derive(Debug, Clone)]
struct NoChunkFound(String);

impl Display for NoChunkFound {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "No chunk with type {} found", self.0)
    }
}

impl std::error::Error for NoChunkFound {}
