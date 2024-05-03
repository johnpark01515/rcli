mod base;
mod csv;
mod genpass;
use std::path::Path;

pub use base::{Base64Format, BaseDecodeOpt, BaseEncodeOpt, BaseSubcmd};
pub use csv::{CsvOpt, Ofmt};

use self::genpass::GenPwdOpt;
use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(name="rcli", version, about, long_about = None)]
pub struct Rcli {
    #[command(subcommand)]
    pub cmd: Option<SubCmd>,
}

#[derive(Debug, Subcommand)]
pub enum SubCmd {
    #[command(name = "csv", about = "Show csv or convert csv to other formats")]
    Csv(CsvOpt),

    #[command(name = "genpass", about = "Gen a password")]
    Genpassword(GenPwdOpt),

    #[command(subcommand)]
    Base64(BaseSubcmd),
}

pub fn parse_input(input: &str) -> Result<String, String> {
    if Path::new(input).exists() || input == "-" {
        Ok(input.into())
    } else {
        Err("File does not exist".into())
    }
}
