use crate::Base64Format;
use anyhow::{Error, Result};
use base64::{
    engine::general_purpose::STANDARD, engine::general_purpose::URL_SAFE_NO_PAD, Engine as _,
};
use std::{io::Read, path::Path};
fn get_reader(input: &str) -> Result<Box<dyn Read>> {
    let reader: Box<dyn Read> = if "-" == input {
        Box::new(std::io::stdin())
    } else {
        Box::new(std::fs::File::open(Path::new(input))?)
    };
    Ok(reader)
}

pub fn base64_encode_process(input: &str, format: Base64Format) -> Result<String, Error> {
    let mut reader = get_reader(input)?;
    let mut encode = Vec::new();
    reader.read_to_end(&mut encode)?;
    let out = match format {
        Base64Format::Standard => STANDARD.encode(encode),
        Base64Format::Urlsafe => URL_SAFE_NO_PAD.encode(encode),
    };
    Ok(out)
}

pub fn base64_decode_process(input: &str, format: Base64Format) -> Result<String, Error> {
    let mut reader = get_reader(input)?;
    let mut decode = String::new();
    reader.read_to_string(&mut decode)?;
    let decode = decode.trim();
    let out = match format {
        Base64Format::Standard => STANDARD.decode(decode)?,
        Base64Format::Urlsafe => URL_SAFE_NO_PAD.decode(decode)?,
    };
    Ok(String::from_utf8(out)?)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_base64_encode_process() -> Result<()> {
        let input = "Cargo.toml";
        let format = Base64Format::Standard;
        assert!(base64_encode_process(input, format).is_ok());
        Ok(())
    }

    #[test]
    fn test_base64_decode_process() -> Result<()> {
        let input = "fixture/base64.txt";
        let format = Base64Format::Standard;
        let out = base64_decode_process(input, format)?;
        let raw = std::fs::read(Path::new("Cargo.toml"))?;
        assert_eq!(out, String::from_utf8(raw)?);
        Ok(())
    }
}
