use serde::{Deserialize, Serialize};

use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::{BufReader, Write, Read};

use crate::item::{Item, Recipe};

#[derive(Serialize, Deserialize, Debug)]
pub struct RecipeBook {
    name: String,
    items: HashMap<String, Item>,
}

impl RecipeBook {
    pub fn new(name: String) -> RecipeBook {
        RecipeBook {
            name: name,
            items: HashMap::new(),
        }
    }

    pub fn path(name: &String) -> String {
        format!("./recipebooks/{}.json", name)
    }

    pub fn load(name: String) -> Result<RecipeBook, anyhow::Error> {
        let path = RecipeBook::path(&name);
        let data = load_file_to_string(&path)?;
        Ok(serde_json::from_str(&data)?)
    }

    pub fn save(&self) -> Result<(), anyhow::Error> {
        let path = RecipeBook::path(&self.name);
        let data = serde_json::to_string_pretty(&self)?;
        std::fs::create_dir_all("recipebooks/")?;
        let mut file = OpenOptions::new().write(true).create(true).open(&path)?;
        file.write_all(&data.as_bytes())?;
        Ok(())
    }
}

pub fn load_file_to_string(path: &str) -> Result<String, anyhow::Error> {
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);
    let mut bytes = Vec::new();
    reader.read_to_end(&mut bytes)?;
    Ok(String::from_utf8(bytes)?)
}