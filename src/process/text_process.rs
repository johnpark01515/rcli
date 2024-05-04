use crate::genpass_process;
use crate::utils::get_writer;
use crate::{utils::get_reader, SignFormat};
use anyhow::Ok;
use anyhow::Result;
use ed25519_dalek::SecretKey;
use ed25519_dalek::Signature;
use ed25519_dalek::Signer;
use ed25519_dalek::SigningKey;
use std::io::Write;
use std::path::Path;
use std::{fs, io::Read};

trait TextSigned {
    fn sign(&self, reader: &mut dyn Read) -> Result<Vec<u8>>;
}

trait TextVerify {
    fn verify(&self, reader: &mut dyn Read, sign: &[u8]) -> Result<bool>;
}

trait GenKeyOutput {
    fn gen_key(&self) -> Result<()>;
}

struct GenKey {
    path: String,
}

impl GenKey {
    fn new(path: &str) -> Self {
        GenKey {
            path: path.to_string(),
        }
    }
}

impl GenKeyOutput for GenKey {
    fn gen_key(&self) -> Result<()> {
        let mut writer = get_writer(&self.path)?;
        let key = genpass_process(32, false, false, false, false)?;
        writer.write_all(key.as_bytes())?;
        Ok(())
    }
}

struct Blake3 {
    key: [u8; 32],
}

struct Ed25519 {
    key: SecretKey,
}
impl TextSigned for Blake3 {
    fn sign(&self, reader: &mut dyn Read) -> Result<Vec<u8>> {
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf)?;
        Ok(blake3::keyed_hash(&self.key, &buf).as_bytes().into())
    }
}

impl TextVerify for Blake3 {
    fn verify(&self, reader: &mut dyn Read, sign: &[u8]) -> Result<bool> {
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf)?;
        let hash = blake3::keyed_hash(&self.key, &buf);
        let signed = hash.as_bytes();
        Ok(sign == signed)
    }
}

impl Blake3 {
    fn new(key: [u8; 32]) -> Self {
        Blake3 { key }
    }

    fn try_new(key: &[u8]) -> Result<Self> {
        let key: [u8; 32] = key.try_into()?;
        Ok(Self::new(key))
    }

    fn load(path: impl AsRef<Path>) -> Result<Self> {
        let res = fs::read(path)?;
        Self::try_new(&res)
    }
}

impl TextSigned for Ed25519 {
    fn sign(&self, reader: &mut dyn Read) -> Result<Vec<u8>> {
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf)?;
        let signing_key = SigningKey::from(self.key);
        let signed = signing_key.sign(&buf);
        Ok(signed.to_bytes().into())
    }
}

impl TextVerify for Ed25519 {
    fn verify(&self, reader: &mut dyn Read, sign: &[u8]) -> Result<bool> {
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf)?;
        let verifyer = SigningKey::from(self.key).verifying_key();
        println!("{}", sign.len());
        let signature = Signature::from_bytes(sign.try_into()?);
        Ok(verifyer.verify_strict(&buf, &signature).is_ok())
    }
}

impl Ed25519 {
    fn new(key: [u8; 32]) -> Self {
        Ed25519 { key }
    }

    fn try_new(key: &[u8]) -> Result<Self> {
        let key: [u8; 32] = key.try_into()?;
        Ok(Self::new(key))
    }

    fn load(path: impl AsRef<Path>) -> Result<Self> {
        let res = fs::read(path)?;
        Self::try_new(&res)
    }
}

pub fn process_sign(input: &str, key: &str, format: SignFormat) -> Result<Vec<u8>> {
    let mut reader = get_reader(input)?;
    let key = fs::read(key)?;
    let signed: Box<dyn TextSigned> = match format {
        SignFormat::Blake3 => Box::new(Blake3::try_new(&key)?),
        SignFormat::Ed25519 => Box::new(Ed25519::try_new(&key)?),
    };
    signed.sign(&mut reader)
}

pub fn process_verify(input: &str, key: &str, format: SignFormat, sign: &[u8]) -> Result<bool> {
    let mut reader = get_reader(input)?;
    let verifier: Box<dyn TextVerify> = match format {
        SignFormat::Blake3 => Box::new(Blake3::load(key)?),
        SignFormat::Ed25519 => Box::new(Ed25519::load(key)?),
    };
    verifier.verify(&mut reader, sign)
}

pub fn process_gen_key(path: &str) -> Result<()> {
    let opt = GenKey::new(path);
    opt.gen_key()
}
