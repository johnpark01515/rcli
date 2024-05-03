use anyhow::Result;
use clap::Parser;
use rcli::{
    base64_decode_process, base64_encode_process, csv_process, genpass_process, BaseSubcmd, Rcli,
    SubCmd,
};

fn main() -> Result<()> {
    let rcli = Rcli::parse();
    match rcli.cmd {
        Some(SubCmd::Csv(opt)) => {
            let output = if let Some(output) = opt.output {
                output
            } else {
                format!("output.{}", opt.format)
            };
            csv_process(&opt.input, opt.format, &output)
        }
        Some(SubCmd::Genpassword(opt)) => {
            genpass_process(
                opt.length,
                opt.noupper,
                opt.nolower,
                opt.nonumber,
                opt.nosymbol,
            )?;
            Ok(())
        }
        Some(SubCmd::Base64(base64)) => {
            match base64 {
                BaseSubcmd::Decode(opt) => {
                    let res = base64_decode_process(&opt.input, opt.format)?;
                    println!("{}", res);
                }
                BaseSubcmd::Encode(opt) => {
                    let res = base64_encode_process(&opt.input, opt.format)?;
                    println!("{}", res);
                }
            }
            Ok(())
        }
        _ => Err(anyhow::anyhow!("No such command")),
    }
}
