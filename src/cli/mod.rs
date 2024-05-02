mod csv;
mod genpass;
pub use csv::{CsvOpt, Ofmt};

use self::csv::Base64Opt;
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

    #[command(name = "b64", about = "use base64 to encode a string or a file")]
    Base64(Base64Opt),
}
