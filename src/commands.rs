use std::fmt::Display;
pub use std::{fs, str::FromStr};

pub use crate::{chunk::Chunk, chunk_type::ChunkType, png::Png, web, Result};

pub mod decode;
pub mod encode;
pub mod print;
pub mod remove;

#[derive(Debug, Clone)]
struct NoChunkFound(String);

impl Display for NoChunkFound {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "No chunk with type {} found", self.0)
    }
}

impl std::error::Error for NoChunkFound {}
