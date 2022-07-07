use crate::{
    args::{DecodeArgs, EncodeArgs, PrintArgs, RemoveArgs},
    chunk::Chunk,
    chunk_type::ChunkType,
    png::Png,
    web, Result,
};
use std::{
    fmt::Display,
    fs::{self},
    str::FromStr,
};

pub mod encode;



pub fn decode_file(args: &DecodeArgs) -> Result<String> {
    let file = &fs::read(&args.file_path)?;
    let png = Png::try_from(&file[..])?;

    let found_chunk = png
        .chunk_by_type(&args.chunk_type)
        .ok_or(NoChunkFound(String::from(&args.chunk_type)))?;

    Ok(found_chunk.data_as_string()?)
}
pub fn remove_encoding(args: &RemoveArgs) -> Result<String> {
    let file = &fs::read(&args.file_path)?;
    let mut png = Png::try_from(&file[..])?;

    let removed_chunk = png.remove_chunk(&args.chunk_type)?;

    let _ = &fs::write(&args.file_path, png.as_bytes())?;

    Ok(removed_chunk.data_as_string()?)
}
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
