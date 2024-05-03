use crate::parse_input;
use anyhow::{Error, Result};
use clap::Parser;
use core::fmt::{self, Display};
use std::str::FromStr;

#[derive(Debug, Parser)]
pub enum BaseSubcmd {
    #[command(name = "encode", about = "encode from stdin or file")]
    Encode(BaseEncodeOpt),

    #[command(name = "decode", about = "decode from stdin or file")]
    Decode(BaseDecodeOpt),
}

#[derive(Debug, Parser)]
pub struct BaseDecodeOpt {
    #[arg(short, long, value_parser = parse_input, default_value = "-")]
    pub input: String,

    #[arg(short, long, default_value = "standard")]
    pub format: Base64Format,
}

#[derive(Debug, Parser)]
pub struct BaseEncodeOpt {
    #[arg(short, long, value_parser = parse_input, default_value = "-")]
    pub input: String,
    #[arg(short, long, default_value = "standard")]
    pub format: Base64Format,
}

#[derive(Debug, Clone, Copy)]
pub enum Base64Format {
    Standard,
    Urlsafe,
}

impl FromStr for Base64Format {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "standard" => Ok(Base64Format::Standard),
            "urlsafe" => Ok(Base64Format::Urlsafe),
            _ => Err(anyhow::anyhow!("Invalid type")),
        }
    }
}

impl From<Base64Format> for &'static str {
    fn from(value: Base64Format) -> Self {
        match value {
            Base64Format::Standard => "standard",
            Base64Format::Urlsafe => "urlsafe",
        }
    }
}

impl Display for Base64Format {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", Into::<&str>::into(*self))
    }
}
