use crate::genpass_process;
use crate::utils::get_writer;
use crate::{utils::get_reader, SignFormat};
use anyhow::{Ok, Result};
use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use base64::Engine;
use chacha20poly1305::{
    aead::{Aead, KeyInit},
    ChaCha20Poly1305,
};
use chacha20poly1305::{AeadCore, Key};
use ed25519_dalek::SecretKey;
use ed25519_dalek::Signature;
use ed25519_dalek::Signer;
use ed25519_dalek::SigningKey;
use rand::rngs::OsRng;
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
        let key: [u8; 32] = key[..32].try_into()?;
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
        let signature = Signature::from_bytes(sign.try_into()?);
        Ok(verifyer.verify_strict(&buf, &signature).is_ok())
    }
}

impl Ed25519 {
    fn new(key: [u8; 32]) -> Self {
        Ed25519 { key }
    }

    fn try_new(key: &[u8]) -> Result<Self> {
        let key: [u8; 32] = key[..32].try_into()?;
        Ok(Self::new(key))
    }

    fn load(path: impl AsRef<Path>) -> Result<Self> {
        let res = fs::read(path)?;
        Self::try_new(&res)
    }
}

pub fn process_sign(input: &str, key: &str, format: SignFormat) -> Result<Vec<u8>> {
    let mut reader = get_reader(input)?;
    // let key = fs::read(key)?;
    let signed: Box<dyn TextSigned> = match format {
        SignFormat::Blake3 => Box::new(Blake3::load(key)?),
        SignFormat::Ed25519 => Box::new(Ed25519::load(key)?),
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

pub struct ChaCha20Poly {
    pub key: Key,
}

impl ChaCha20Poly {
    fn new(key: [u8; 32]) -> Self {
        ChaCha20Poly { key: key.into() }
    }

    fn try_new(key: &[u8]) -> Result<Self> {
        let k: [u8; 32] = key[..32].try_into()?;
        Ok(Self::new(k))
    }

    fn load(key_path: impl AsRef<Path>) -> Result<Self> {
        let key = fs::read(key_path)?;
        Self::try_new(&key)
    }
}

trait Encrypt {
    fn encrypt(&self, reader: &mut dyn Read) -> Result<String>;
}

trait Decrypt {
    fn decrypt(&self, reader: &mut dyn Read) -> Result<String>;
}

impl Encrypt for ChaCha20Poly {
    fn encrypt(&self, reader: &mut dyn Read) -> Result<String> {
        let mut buf = String::new();
        reader.read_to_string(&mut buf)?;
        let buf = buf.trim();
        let cipher = ChaCha20Poly1305::new(&self.key);
        let nonce = ChaCha20Poly1305::generate_nonce(&mut OsRng); // 192-bits; unique per message
        let ciphertext = cipher.encrypt(&nonce, buf.as_ref())?;
        Ok(format!(
            "{}|{}",
            URL_SAFE_NO_PAD.encode(ciphertext),
            URL_SAFE_NO_PAD.encode(nonce)
        ))
    }
}

impl Decrypt for ChaCha20Poly {
    fn decrypt(&self, reader: &mut dyn Read) -> Result<String> {
        let mut buf = String::new();
        reader.read_to_string(&mut buf)?;
        let index = buf.find('|').unwrap();

        let encrypted = URL_SAFE_NO_PAD.decode(&buf[..index])?;
        let nonce = URL_SAFE_NO_PAD.decode(&buf[index + 1..])?;
        let nonce: [u8; 12] = nonce[..12].try_into()?;
        let cipher = ChaCha20Poly1305::new(&self.key);
        let plaintext = cipher.decrypt(&nonce.into(), encrypted.as_ref())?;
        Ok(String::from_utf8(plaintext)?)
    }
}

pub fn process_text_encrypt(key: &str, input: &str) -> Result<String> {
    let chacha = ChaCha20Poly::load(key)?;
    let mut reader = get_reader(input)?;
    chacha.encrypt(&mut reader)
}

pub fn process_text_decrype(key: &str, enctypted: &str) -> Result<String> {
    let chacha = ChaCha20Poly::load(key)?;
    let mut reader = get_reader(enctypted)?;
    chacha.decrypt(&mut reader)
    // Ok("123".into())
}
