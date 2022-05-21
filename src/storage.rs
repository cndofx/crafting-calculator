use serde::{Deserialize, Serialize};

use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::{BufReader, Write, Read};

use crate::item::Item;

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    recipe_files: Vec<String>
}

impl Config {
    pub fn new() -> Config {
        Config { recipe_files: Vec::new() }
    }

    pub fn from_file() -> Result<Config, anyhow::Error> {
        let data = load_file_to_string("config.json")?;
        Ok(serde_json::from_str(&data)?)
    }

    pub fn save_file(config: &Config) -> Result<(), anyhow::Error> {
        let data = serde_json::to_string_pretty(&config)?;
        let mut file = OpenOptions::new().write(true).create(true).open("config.json")?;
        file.write_all(&data.as_bytes())?;
        Ok(())
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Items {
    items: HashMap<String, Item>
}

pub fn load_file_to_string(path: &str) -> Result<String, anyhow::Error> {
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);
    let mut bytes = Vec::new();
    reader.read_to_end(&mut bytes)?;
    Ok(String::from_utf8(bytes)?)
}