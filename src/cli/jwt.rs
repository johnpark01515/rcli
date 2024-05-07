use crate::{get_reader, parse_file, process_jwt_sign, process_jwt_verify, CmdExecutor};
use anyhow::Result;
use clap::Parser;
use enum_dispatch::enum_dispatch;
use regex::Regex;
use std::time::SystemTime;

#[derive(Debug, Parser)]
#[enum_dispatch(CmdExecutor)]
pub enum JwtSubcmd {
    #[command(about = "sign to gen a jwt")]
    Sign(JwtSignOpt),

    #[command(about = "verify a token")]
    Verify(JwtVerifyOpt),
}

#[derive(Debug, Parser)]
pub struct JwtSignOpt {
    #[arg(short, long, default_value = "acme")]
    sub: String,

    #[arg(short, long, default_value = "device1")]
    aud: String,

    #[arg(short, long, value_parser=parse_exp, default_value = "1d")]
    exp: u64,
}

impl CmdExecutor for JwtSignOpt {
    async fn execute(self) -> Result<()> {
        let res = process_jwt_sign(&self.sub, &self.aud, self.exp).await?;
        println!("{}", res);
        Ok(())
    }
}

#[derive(Debug, Parser)]
pub struct JwtVerifyOpt {
    #[arg(short, long,value_parser = parse_file, default_value = "-")]
    token: String,
}

impl CmdExecutor for JwtVerifyOpt {
    async fn execute(self) -> Result<()> {
        let mut reader = get_reader(&self.token)?;
        let res = process_jwt_verify(&mut reader).await?;
        println!("{}", res);
        Ok(())
    }
}

fn parse_exp(exp: &str) -> Result<u64> {
    let now = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)?
        .as_secs();
    let added = parse_exp_string(exp.to_string())?;
    Ok(now + added)
}

fn parse_exp_string(exp: String) -> Result<u64> {
    let re = Regex::new(r"^(?<num>\d+)(?i)(?<unit>s|m|h|d)$")?;
    let res = re.captures(&exp).unwrap();
    let num: u64 = res["num"].to_string().parse()?;
    let unit = &res["unit"];
    let exp = match unit {
        "s" | "S" => num,
        "m" | "M" => num * 60,
        "h" | "H" => num * 3600,
        "d" | "D" => num * 3600 * 24,
        _ => return Err(anyhow::anyhow!("Invalid unit for exp")),
    };
    Ok(exp)
}
