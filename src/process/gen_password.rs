use anyhow::Result;
use rand::prelude::*;

const UPPER: &[u8] = b"ABCDEFGHIGKLMNPQRSTUVWSYZ";
const LOWER: &[u8] = b"abcdefghigkmnopqrstuvwsyz";
const NUMBER: &[u8] = b"123456789";
const SYMBOL: &[u8] = b"!@#$%^&*";

pub async fn genpass_process(
    length: u8,
    noupper: bool,
    nolower: bool,
    nonumber: bool,
    nosymbol: bool,
) -> Result<String, anyhow::Error> {
    let mut res = Vec::with_capacity(length as usize);
    let mut rng = thread_rng();
    let mut shadow = Vec::with_capacity(70);
    if !nolower {
        res.push(*LOWER.choose(&mut rng).unwrap());
        shadow.extend_from_slice(LOWER);
    }
    if !noupper {
        res.push(*UPPER.choose(&mut rng).unwrap());
        shadow.extend_from_slice(UPPER);
    }
    if !nonumber {
        res.push(*NUMBER.choose(&mut rng).unwrap());
        shadow.extend_from_slice(NUMBER);
    }
    if !nosymbol {
        res.push(*SYMBOL.choose(&mut rng).unwrap());
        shadow.extend_from_slice(SYMBOL);
    }
    for _ in 0..(length as usize - res.len()) {
        res.push(*shadow.choose(&mut rng).unwrap());
    }
    res.shuffle(&mut rng);
    let password = String::from_utf8(res)?;
    Ok(password)
}
