use anyhow::{Error, Result};
use clap::Parser;
use std::fmt::Display;
use std::path::Path;
use std::str::FromStr;

#[derive(Debug, Parser)]
pub struct Base64Opt {}

#[derive(Parser, Debug)]
pub struct CsvOpt {
    #[arg(short, long, value_parser=parse_input)]
    pub input: String,

    #[arg(short, long)]
    pub output: Option<String>,

    #[arg(short, long, default_value = "json")]
    pub format: Ofmt,

    #[arg(short, long, default_value_t = ',')]
    pub delimiter: char,

    #[arg(long, default_value_t = true)]
    pub header: bool,
}

pub fn parse_input(input: &str) -> Result<String, String> {
    if Path::new(input).exists() {
        Ok(input.into())
    } else {
        Err("File does not exist".into())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Ofmt {
    Json,
    Yaml,
    Toml,
}

impl FromStr for Ofmt {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "json" => Ok(Ofmt::Json),
            "yaml" => Ok(Ofmt::Yaml),
            "toml" => Ok(Ofmt::Toml),
            _ => Err(anyhow::anyhow!("Invalid format")),
        }
    }
}

impl From<Ofmt> for &'static str {
    fn from(value: Ofmt) -> Self {
        match value {
            Ofmt::Json => "json",
            Ofmt::Yaml => "yaml",
            Ofmt::Toml => "toml",
        }
    }
}
impl Display for Ofmt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Into::<&str>::into(*self))
    }
}
