use crate::{http_process_serve, parse_file, CmdExecutor};
use anyhow::Result;
use clap::{Parser, Subcommand};

#[derive(Debug, Subcommand)]
pub enum HttpSubcmd {
    #[command(about = "Serve a Dir for http")]
    Serve(ServeOpt),
}

impl CmdExecutor for HttpSubcmd {
    async fn execute(self) -> Result<()> {
        match self {
            HttpSubcmd::Serve(opt) => opt.execute().await,
        }
    }
}

#[derive(Debug, Parser)]
pub struct ServeOpt {
    #[arg(short, long, value_parser=parse_file)]
    pub dir: String,

    #[arg(short, long, default_value_t = 8080)]
    pub port: u16,
}

impl CmdExecutor for ServeOpt {
    async fn execute(self) -> Result<()> {
        http_process_serve((&self.dir).into(), self.port).await
    }
}
