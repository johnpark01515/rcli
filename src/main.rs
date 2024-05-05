use anyhow::Result;
use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};
use clap::Parser;
use rcli::{
    base64_decode_process, base64_encode_process, csv_process, genpass_process, http_process_serve,
    process_gen_key, process_sign, process_text_decrype, process_text_encrypt, process_verify,
    BaseSubcmd, HttpSubcmd, Rcli, SubCmd, TextSubcmd,
};

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    let rcli = Rcli::parse();
    match rcli.cmd {
        Some(SubCmd::Csv(opt)) => {
            let output = if let Some(output) = opt.output {
                output
            } else {
                format!("output.{}", opt.format)
            };
            csv_process(&opt.input, opt.format, &output)?
        }
        Some(SubCmd::Genpassword(opt)) => {
            genpass_process(
                opt.length,
                opt.noupper,
                opt.nolower,
                opt.nonumber,
                opt.nosymbol,
            )?;
        }
        Some(SubCmd::Base64(base64)) => match base64 {
            BaseSubcmd::Decode(opt) => {
                base64_decode_process(&opt.input, opt.format)?;
            }
            BaseSubcmd::Encode(opt) => {
                base64_encode_process(&opt.input, opt.format)?;
            }
        },
        Some(SubCmd::Text(sub_cmd)) => match sub_cmd {
            TextSubcmd::Sign(opt) => {
                let res = process_sign(&opt.input, &opt.key, opt.format)?;
                let encoded = URL_SAFE_NO_PAD.encode(res);
                println!("{}", encoded);
            }
            TextSubcmd::Verify(opt) => {
                let sign = URL_SAFE_NO_PAD.decode(opt.sig)?;
                let res = process_verify(&opt.input, &opt.key, opt.format, &sign)?;
                println!("{}", res);
            }
            TextSubcmd::Genkey(opt) => process_gen_key(&opt.output)?,
            TextSubcmd::Encrypt(opt) => {
                let res = process_text_encrypt(&opt.key, &opt.input)?;
                print!("{}", res);
            }

            TextSubcmd::Decrypt(opt) => {
                let res = process_text_decrype(&opt.key, &opt.input)?;
                print!("decrypt:{}", res);
            }
        },
        Some(SubCmd::Http(sub_cmd)) => match sub_cmd {
            HttpSubcmd::Serve(opt) => http_process_serve((&opt.dir).into(), opt.port).await?,
        },
        _ => Err(anyhow::anyhow!("No such command"))?,
    };
    Ok(())
}
