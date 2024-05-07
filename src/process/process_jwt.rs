use anyhow::Result;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::io::Read;

const SECRET: &str = "jwttokensecret";

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    aud: String,
    exp: u64,
}

pub async fn process_jwt_sign(sub: &str, aud: &str, exp: u64) -> Result<String> {
    let claims = Claims {
        sub: sub.to_owned(),
        aud: aud.to_owned(),
        exp,
    };
    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(SECRET.as_ref()),
    )?;
    Ok(token)
}

pub async fn process_jwt_verify(reader: &mut dyn Read) -> Result<String> {
    let mut token = String::new();
    reader.read_to_string(&mut token)?;
    let token = token.trim().to_string();
    let mut validation = Validation::new(Algorithm::HS256);
    validation.validate_aud = false;
    let token_message = decode::<Claims>(
        &token,
        &DecodingKey::from_secret(SECRET.as_ref()),
        &validation,
    )?;
    Ok(serde_json::to_string_pretty(&token_message.claims)?)
}
