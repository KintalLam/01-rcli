use csv::Reader;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fs;
use anyhow::Result;

#[derive(Debug,  Deserialize, Serialize)]
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

pub fn process_csv(input: &str, output: &str) -> Result<()>{
    let mut reader = Reader::from_path(input)?;
    let mut ret = Vec::with_capacity(128);
    let header = reader.headers()?.clone();
    for result in reader.records(){
        let record = result?;
        let json_value = header.iter().zip(record.iter()).collect::<Value>();
        ret.push(json_value)
    }

    let json = serde_json::to_string_pretty(&ret)?;
    fs::write(output, json)?;
    Ok(())
}
