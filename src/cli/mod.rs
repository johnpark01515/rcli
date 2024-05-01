mod csv;
pub use csv::{CsvOpt, Ofmt};

use self::csv::Base64Opt;
use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(name="rcli", version, about, long_about = None)]
pub struct Rcli {
    #[command(subcommand)]
    pub cmd: Option<SubCmd>,
}

#[derive(Debug, Subcommand)]
pub enum SubCmd {
    #[command(name = "csv", about = "translate csv to other format")]
    Csv(CsvOpt),

    #[command(name = "b64", about = "use base64 to encode a string or a file")]
    Base64(Base64Opt),
}
