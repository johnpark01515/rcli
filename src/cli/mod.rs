mod base;
mod csv;
mod genpass;
mod http;
mod text;

pub use self::genpass::GenPwdOpt;
pub use base::*;
use clap::{Parser, Subcommand};
pub use csv::{CsvOpt, Ofmt};
use enum_dispatch::enum_dispatch;
pub use http::*;
pub use text::*;

#[derive(Debug, Parser)]
#[command(name="rcli", version, about, long_about = None)]
pub struct Rcli {
    #[command(subcommand)]
    pub cmd: SubCmd,
}

#[derive(Debug, Subcommand)]
#[enum_dispatch(CmdExecutor)]
pub enum SubCmd {
    #[command(name = "csv", about = "Show csv or convert csv to other formats")]
    Csv(CsvOpt),

    #[command(name = "genpass", about = "Gen a password")]
    Genpassword(GenPwdOpt),

    #[command(subcommand)]
    Base64(BaseSubcmd),

    #[command(subcommand)]
    Text(TextSubcmd),

    #[command(subcommand)]
    Http(HttpSubcmd),
}
