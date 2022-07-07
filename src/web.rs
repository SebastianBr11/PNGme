use bytes::Bytes;

use super::Result;

pub fn get_file_from(url: &str) -> Result<Bytes> {
    let res = reqwest::blocking::get(url)?;
    Ok(res.bytes()?)
}
