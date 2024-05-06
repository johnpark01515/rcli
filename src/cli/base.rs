use crate::{base64_decode_process, base64_encode_process, parse_file, CmdExecutor};
use anyhow::Result;
use clap::Parser;
use core::fmt::{self, Display};
use enum_dispatch::enum_dispatch;
use std::str::FromStr;

#[derive(Debug, Parser)]
#[enum_dispatch(CmdExecutor)]
pub enum BaseSubcmd {
    #[command(name = "encode", about = "encode from stdin or file")]
    Encode(BaseEncodeOpt),

    #[command(name = "decode", about = "decode from stdin or file")]
    Decode(BaseDecodeOpt),
}

#[derive(Debug, Parser)]
pub struct BaseDecodeOpt {
    #[arg(short, long, value_parser = parse_file, default_value = "-")]
    pub input: String,

    #[arg(short, long, default_value = "standard")]
    pub format: Base64Format,
}

impl CmdExecutor for BaseDecodeOpt {
    async fn execute(self) -> Result<()> {
        let res = base64_decode_process(&self.input, self.format).await?;
        println!("{}", res);
        Ok(())
    }
}

#[derive(Debug, Parser)]
pub struct BaseEncodeOpt {
    #[arg(short, long, value_parser = parse_file, default_value = "-")]
    pub input: String,
    #[arg(short, long, default_value = "standard")]
    pub format: Base64Format,
}

impl CmdExecutor for BaseEncodeOpt {
    async fn execute(self) -> Result<()> {
        let res = base64_encode_process(&self.input, self.format).await?;
        println!("{}", res);
        Ok(())
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Base64Format {
    Standard,
    Urlsafe,
}

impl FromStr for Base64Format {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "standard" => Ok(Base64Format::Standard),
            "urlsafe" => Ok(Base64Format::Urlsafe),
            _ => Err(anyhow::anyhow!("Invalid type")),
        }
    }
}

impl From<Base64Format> for &'static str {
    fn from(value: Base64Format) -> Self {
        match value {
            Base64Format::Standard => "standard",
            Base64Format::Urlsafe => "urlsafe",
        }
    }
}

impl Display for Base64Format {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", Into::<&str>::into(*self))
    }
}
