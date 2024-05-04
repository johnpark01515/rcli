use anyhow::Result;
use std::{
    fs::File,
    io::{self, Read, Write},
    path::Path,
};

pub fn parse_file(input: &str) -> Result<String, String> {
    if Path::new(input).exists() || input == "-" {
        Ok(input.into())
    } else {
        Err("File does not exist".into())
    }
}

pub fn parse_out(output: &str) -> Result<String, String> {
    let path = Path::new(output);
    if path.is_dir() {
        Err("Path is dir".into())
    } else {
        Ok(output.into())
    }
}

pub fn get_reader(input: &str) -> Result<Box<dyn Read>> {
    let reader: Box<dyn Read> = if "-" == input {
        Box::new(io::stdin())
    } else {
        Box::new(File::open(Path::new(input))?)
    };
    Ok(reader)
}

pub fn get_writer(output: &str) -> Result<Box<dyn Write>> {
    let writer: Box<dyn Write> = if "-" == output {
        Box::new(io::stdout())
    } else {
        let path = Path::new(output);
        let file = File::create(path)?;
        Box::new(file)
    };
    Ok(writer)
}
