use anyhow::Result;
use clap::Parser;
use rcli::{csv_process, Rcli, SubCmd};

fn main() -> Result<()> {
    let rcli = Rcli::parse();
    match rcli.cmd {
        Some(SubCmd::Csv(opt)) => csv_process(opt.input, opt.format, opt.output),
        Some(SubCmd::Base64(base64)) => {
            print!("{:?}", base64);
            Ok(())
        }
        _ => Err(anyhow::anyhow!("No such command")),
    }
}
