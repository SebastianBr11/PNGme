use super::*;
use crate::args::RemoveArgs;

pub struct Remove<'a> {
    args: &'a RemoveArgs,
}

impl<'a> Remove<'a> {
    pub fn new(args: &RemoveArgs) -> Remove {
        Remove { args }
    }

    pub fn remove_web_encoding(&self) -> Result<String> {
        let file = web::get_file_from(&self.args.file_path)?;

        let (png, removed_message) = self.remove_encoding(&file)?;
        if let Some(file_path) = &self.args.output_file {
            let _ = &fs::write(file_path, png.as_bytes())?;
        } else {
            println!("Missing output file path!");
        }
        Ok(removed_message)
    }

    pub fn remove_local_encoding(&self) -> Result<String> {
        let file = &fs::read(&self.args.file_path)?;
        let (png, removed_message) = self.remove_encoding(file)?;

        if let Some(file_path) = &self.args.output_file {
            let _ = &fs::write(file_path, png.as_bytes())?;
        } else {
            let _ = &fs::write(&self.args.file_path, png.as_bytes())?;
        }

        Ok(removed_message)
    }

    fn remove_encoding(&self, file_data: &[u8]) -> Result<(Png, String)> {
        let mut png = Png::try_from(file_data)?;
        let removed_chunk = png.remove_chunk(&self.args.chunk_type)?;

        Ok((png, removed_chunk.data_as_string()?))
    }
}
