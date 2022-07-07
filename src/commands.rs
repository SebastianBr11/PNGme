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

pub struct Encode<'a> {
    args: &'a EncodeArgs,
}

impl<'a> Encode<'a> {
    pub fn new(args: &EncodeArgs) -> Encode {
        Encode { args }
    }

    pub fn encode_web_file(&self) -> Result<()> {
        let file = web::get_file_from(&self.args.file_path)?;

        let png = self.encode_file(&file)?;
        if let Some(file_path) = &self.args.output_file {
            let _ = &fs::write(file_path, png.as_bytes())?;
        } else {
            println!("Missing output file path!");
        }
        Ok(())
    }

    pub fn encode_local_file(&self) -> Result<()> {
        let file = &fs::read(&self.args.file_path)?;
        let png = self.encode_file(file)?;

        if let Some(file_path) = &self.args.output_file {
            let _ = &fs::write(file_path, png.as_bytes())?;
        } else {
            let _ = &fs::write(&self.args.file_path, png.as_bytes())?;
        }
        Ok(())
    }

    fn encode_file(&self, file_data: &[u8]) -> Result<Png> {
        let mut png = Png::try_from(&file_data[..])?;
        let data: Vec<_> = self.args.message[..].as_bytes().iter().copied().collect();

        png.append_chunk(Chunk::new(
            ChunkType::from_str(&self.args.chunk_type)?,
            data,
        ));

        Ok(png)
    }
}

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
