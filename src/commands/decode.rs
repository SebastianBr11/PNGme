use super::*;
use crate::args::DecodeArgs;

pub struct Decode<'a> {
    args: &'a DecodeArgs,
}

impl<'a> Decode<'a> {
    pub fn new(args: &DecodeArgs) -> Decode {
        Decode { args }
    }

    pub fn decode(&self) -> Result<String> {
        if self.args.web {
            self.decode_web_file()
        } else {
            self.decode_local_file()
        }
    }

    fn decode_web_file(&self) -> Result<String> {
        let file = web::get_file_from(&self.args.file_path)?;
        self.decode_file(&file)
    }

    fn decode_local_file(&self) -> Result<String> {
        let file = &fs::read(&self.args.file_path)?;
        self.decode_file(file)
    }

    fn decode_file(&self, file_data: &[u8]) -> Result<String> {
        let png = Png::try_from(file_data)?;

        let found_chunk = png
            .chunk_by_type(&self.args.chunk_type)
            .ok_or(NoChunkFound(String::from(&self.args.chunk_type)))?;

        found_chunk.data_as_string()
    }
}
