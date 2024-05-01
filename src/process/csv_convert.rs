use crate::Ofmt;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::{fs, path::Path};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
struct Player {
    name: String,
    position: String,
    #[serde(rename = "DOB")]
    dob: String,
    nationality: String,
    #[serde(rename = "Kit Number")]
    kit: u8,
}

pub fn csv_process(path: String, format: Ofmt, output: String) -> Result<()> {
    let file_path = Path::new(&path);
    if !file_path.exists() {
        return Err(anyhow::anyhow!("path not exist"));
    }
    let mut reader = csv::Reader::from_path(file_path)?;
    let header = reader.headers().expect("get header error").clone();
    let mut res = Vec::with_capacity(128);
    for i in reader.records() {
        let record = i?;
        let json_value = header
            .iter()
            .zip(record.iter())
            .collect::<serde_json::Value>();
        res.push(json_value);
    }
    let content = match format {
        Ofmt::Json => serde_json::to_string_pretty(&res)?,
        Ofmt::Yaml => serde_yaml::to_string(&res)?,
    };
    fs::write(Path::new(&output), content)?;
    Ok(())
}
