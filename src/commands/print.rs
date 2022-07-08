use super::*;
use crate::args::PrintArgs;

pub struct Print<'a> {
    args: &'a PrintArgs,
}

impl<'a> Print<'a> {
    pub fn new(args: &PrintArgs) -> Print {
        Print { args }
    }

    pub fn print_web_chunks(&self) -> Result<()> {
        let file = web::get_file_from(&self.args.file_path)?;
        self.print_chunks(&file)
    }

    pub fn print_local_chunks(&self) -> Result<()> {
        let file = &fs::read(&self.args.file_path)?;
        self.print_chunks(file)
    }

    fn print_chunks(&self, file_data: &[u8]) -> Result<()> {
        let png = Png::try_from(&file_data[..])?;

        println!("{png}");

        Ok(())
    }
}
