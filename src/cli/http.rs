use crate::utils::parse_file;
use clap::{Parser, Subcommand};

#[derive(Debug, Subcommand)]
pub enum HttpSubcmd {
    #[command(about = "Serve a Dir for http")]
    Serve(ServeOpt),
}

#[derive(Debug, Parser)]
pub struct ServeOpt {
    #[arg(short, long, value_parser=parse_file)]
    pub dir: String,

    #[arg(short, long, default_value_t = 8080)]
    pub port: u16,
}
