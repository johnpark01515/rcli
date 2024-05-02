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

pub fn csv_process(path: &str, format: Ofmt, output: &str) -> Result<()> {
    let file_path = Path::new(path);
    if !file_path.exists() {
        return Err(anyhow::anyhow!("file not exist"));
    }
    let mut reader = csv::Reader::from_path(file_path)?;
    let res = reader
        .deserialize()
        .map(|r| r.unwrap())
        .collect::<Vec<Player>>();

    let content = match format {
        Ofmt::Json => serde_json::to_string_pretty(&res)?,
        Ofmt::Yaml => serde_yaml::to_string(&res)?,
    };
    fs::write(Path::new(&format!("{}.{}", output, format)), content)?;
    Ok(())
}
