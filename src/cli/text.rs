use crate::{
    parse_file, parse_out, process_gen_key, process_sign, process_text_decrypt,
    process_text_encrypt, process_verify, CmdExecutor,
};
use anyhow::{Error, Result};
use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};
use clap::{Parser, Subcommand};
use enum_dispatch::enum_dispatch;
use std::{
    fmt::{self, Display},
    str::FromStr,
};

#[derive(Debug, Subcommand)]
#[enum_dispatch(CmdExecutor)]
pub enum TextSubcmd {
    #[command(about = "Sign a text with a private/session key and return a signature")]
    Sign(SignOpt),

    #[command(about = "Verify a signature with a public/session key")]
    Verify(VerifyOpt),

    #[command(about = "Generate a key for blake3/ed25519")]
    Genkey(GenKeyOpt),

    #[command(about = "Encrypt a text and put out base64")]
    Encrypt(EncryptOpt),

    #[command(about = "Encrypt a base64 text and put out raw text")]
    Decrypt(DecryptOpt),
}

#[derive(Debug, Parser)]
pub struct EncryptOpt {
    #[arg(short, long, value_parser=parse_file)]
    pub key: String,

    #[arg(short, long, value_parser=parse_file, default_value="-")]
    pub input: String,
}

impl CmdExecutor for EncryptOpt {
    async fn execute(self) -> Result<()> {
        let res = process_text_encrypt(&self.key, &self.input).await?;
        print!("{}", res);
        Ok(())
    }
}

#[derive(Debug, Parser)]
pub struct DecryptOpt {
    #[arg(short, long, value_parser=parse_file)]
    pub key: String,

    #[arg(short, long, value_parser=parse_file, default_value="-")]
    pub input: String,
}

impl CmdExecutor for DecryptOpt {
    async fn execute(self) -> Result<()> {
        let res = process_text_decrypt(&self.key, &self.input).await?;
        print!("{}", res);
        Ok(())
    }
}

#[derive(Debug, Parser)]
pub struct GenKeyOpt {
    #[arg(short, long, value_parser = parse_out, default_value = "-")]
    pub output: String,
}

impl CmdExecutor for GenKeyOpt {
    async fn execute(self) -> Result<()> {
        process_gen_key(&self.output).await
    }
}

#[derive(Debug, Parser)]
pub struct SignOpt {
    #[arg(short, long, value_parser=parse_file, default_value = "-")]
    pub input: String,

    #[arg(short, long)]
    pub key: String,

    #[arg(short, long, default_value = "blake3")]
    pub format: SignFormat,
}

impl CmdExecutor for SignOpt {
    async fn execute(self) -> Result<()> {
        let res = process_sign(&self.input, &self.key, self.format).await?;
        let encoded = URL_SAFE_NO_PAD.encode(res);
        println!("{}", encoded);
        Ok(())
    }
}

#[derive(Debug, Parser)]
pub struct VerifyOpt {
    #[arg(short, long, value_parser=parse_file, default_value = "-")]
    pub input: String,

    #[arg(short, long)]
    pub key: String,

    #[arg(short, long, default_value = "blake3")]
    pub format: SignFormat,

    #[arg(short, long)]
    pub sig: String,
}

impl CmdExecutor for VerifyOpt {
    async fn execute(self) -> Result<()> {
        let sign = URL_SAFE_NO_PAD.decode(self.sig)?;
        let res = process_verify(&self.input, &self.key, self.format, &sign).await?;
        println!("{}", res);
        Ok(())
    }
}

#[derive(Debug, Clone, Copy)]
pub enum SignFormat {
    Blake3,
    Ed25519,
}

impl FromStr for SignFormat {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "blake3" => Ok(SignFormat::Blake3),
            "ed25519" => Ok(SignFormat::Ed25519),
            _ => Err(anyhow::anyhow!("Invalid format")),
        }
    }
}

impl From<SignFormat> for &'static str {
    fn from(value: SignFormat) -> Self {
        match value {
            SignFormat::Blake3 => "blake3",
            SignFormat::Ed25519 => "ed25519",
        }
    }
}

impl Display for SignFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", Into::<&str>::into(*self))
    }
}
