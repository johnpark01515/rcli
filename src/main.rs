use anyhow::{Error, Result};
use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use std::{fs, path::Path, str::FromStr};

#[derive(Debug, Parser)]
#[command(name="rcli", version, about, long_about = None)]
struct Rcli {
    #[command(subcommand)]
    cmd: Option<SubCmd>,
}

#[derive(Debug, Subcommand)]
enum SubCmd {
    #[command(name = "csv", about = "translate csv to other format")]
    Csv(CsvOpt),

    #[command(name = "b64", about = "use base64 to encode a string or a file")]
    Base64(Base64Opt),
}

#[derive(Debug, Parser)]
struct Base64Opt {}

#[derive(Parser, Debug)]
struct CsvOpt {
    #[arg(short, long, help = "the csv file to translate")]
    input: String,

    #[arg(
        short,
        long,
        help = "the file to translate into",
        default_value = "output"
    )]
    output: String,

    #[arg(
        short,
        long,
        help = "the output format: json/yaml",
        default_value = "json"
    )]
    format: Ofmt,

    #[arg(short, long, default_value_t = ',')]
    delimiter: char,

    #[arg(
        long,
        help = "whether or not has header for the csv data",
        default_value_t = true
    )]
    header: bool,
}

#[derive(Debug, Clone)]
enum Ofmt {
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

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
struct Player {
    name: String,
    position: String,
    #[serde(rename = "DOB")]
    dob: String,
    nationality: String,
    #[serde(rename = "Kit Number")]
    kit: u8,
}

fn main() {
    let rcli = Rcli::parse();
    match rcli.cmd {
        Some(SubCmd::Csv(opt)) => {
            let file_path = Path::new(opt.input.as_str());
            if !file_path.exists() {
                return println!("file:{:?} not exists", file_path);
            }
            let mut reader = csv::Reader::from_path(file_path).expect("Read file error");
            let header = reader.headers().expect("get header error").clone();
            let mut res = Vec::with_capacity(128);
            for i in reader.records() {
                let record = i.unwrap();
                let json_value = header
                    .iter()
                    .zip(record.iter())
                    .collect::<serde_json::Value>();
                res.push(json_value);
            }
            let content = match opt.format {
                Ofmt::Json => serde_json::to_string_pretty(&res).expect("parse json error"),
                Ofmt::Yaml => serde_yaml::to_string(&res).expect("parse yaml error"),
            };
            fs::write(Path::new(&opt.output), content).unwrap();
        }
        Some(SubCmd::Base64(base64)) => {
            println!("base: {:?}", base64)
        }
        _ => {}
    }
}
