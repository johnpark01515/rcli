use crate::Ofmt;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs, path::Path};

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

pub async fn csv_process(path: &str, format: Ofmt, output: &str) -> Result<()> {
    let file_path = Path::new(path);
    if !file_path.exists() {
        return Err(anyhow::anyhow!("file not exist"));
    }
    let mut reader = csv::Reader::from_path(file_path)?;
    let header = reader.headers()?.clone();
    let mut res = Vec::with_capacity(128);
    for record in reader.records() {
        let record = record?;
        let value = header
            .iter()
            .zip(record.iter())
            .collect::<serde_json::Value>();
        res.push(value);
    }

    let content = match format {
        Ofmt::Json => serde_json::to_string_pretty(&res)?,
        Ofmt::Yaml => serde_yaml::to_string(&res)?,
        Ofmt::Toml => {
            let mut tomal_map = HashMap::new();
            for i in res {
                let name = i
                    .get("Name")
                    .unwrap()
                    .to_string()
                    .trim_matches('"')
                    .to_string();
                tomal_map.insert(name, i);
            }
            toml::to_string_pretty(&tomal_map)?
        }
    };
    fs::write(Path::new(output), content)?;
    Ok(())
}
