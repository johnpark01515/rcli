mod base;
mod csv;
mod genpass;
mod text;

use self::genpass::GenPwdOpt;
pub use base::{Base64Format, BaseDecodeOpt, BaseEncodeOpt, BaseSubcmd};
use clap::{Parser, Subcommand};
pub use csv::{CsvOpt, Ofmt};
pub use text::{SignFormat, TextSubcmd};

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

    #[command(subcommand)]
    Text(TextSubcmd),
}
