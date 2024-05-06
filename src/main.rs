use anyhow::Result;
use clap::Parser;
use rcli::{CmdExecutor, Rcli};

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    let rcli = Rcli::parse();
    rcli.cmd.execute().await
}
