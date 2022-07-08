use super::*;
use crate::args::EncodeArgs;

pub struct Encode<'a> {
    args: &'a EncodeArgs,
}

impl<'a> Encode<'a> {
    pub fn new(args: &EncodeArgs) -> Encode {
        Encode { args }
    }

    pub fn encode(&self) -> Result<()> {
        if self.args.web {
            self.encode_web_file()
        } else {
            self.encode_local_file()
        }
    }

    fn encode_web_file(&self) -> Result<()> {
        let file = web::get_file_from(&self.args.file_path)?;

        let png = self.encode_file(&file)?;
        if let Some(file_path) = &self.args.output_file {
            let _ = &fs::write(file_path, png.as_bytes())?;
        } else {
            println!("Missing output file path!");
        }
        Ok(())
    }

    fn encode_local_file(&self) -> Result<()> {
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
