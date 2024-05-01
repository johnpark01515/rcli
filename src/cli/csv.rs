use anyhow::{Error, Result};
use clap::Parser;
use std::str::FromStr;

#[derive(Debug, Parser)]
pub struct Base64Opt {}

#[derive(Parser, Debug)]
pub struct CsvOpt {
    #[arg(short, long, help = "the csv file to translate")]
    pub input: String,

    #[arg(
        short,
        long,
        help = "the file to translate into",
        default_value = "output"
    )]
    pub output: String,

    #[arg(
        short,
        long,
        help = "the output format: json/yaml",
        default_value = "json"
    )]
    pub format: Ofmt,

    #[arg(short, long, default_value_t = ',')]
    pub delimiter: char,

    #[arg(
        long,
        help = "whether or not has header for the csv data",
        default_value_t = true
    )]
    pub header: bool,
}

#[derive(Debug, Clone)]
pub enum Ofmt {
    Json,
    Yaml,
}

impl FromStr for Ofmt {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "json" => Ok(Ofmt::Json),
            "yaml" => Ok(Ofmt::Yaml),
            _ => Err(anyhow::anyhow!("Invalid format")),
        }
    }
}

impl From<Ofmt> for &'static str {
    fn from(value: Ofmt) -> Self {
        match value {
            Ofmt::Json => "json",
            Ofmt::Yaml => "yaml",
        }
    }
}
