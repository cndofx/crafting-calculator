use serde::{Deserialize, Serialize};

use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::{BufReader, Read, Write};

use crate::item::{Item, Recipe};

/// A `RecipeBook` is a named collection of recipes that can be
/// saved to and loaded from disk.
#[derive(Serialize, Deserialize, Debug)]
pub struct RecipeBook {
    /// Name of the `RecipeBook`, used both for display to the user
    /// and for the filename. Spaces are allowed.
    name: String,
    /// `HashMap` of all items defined in the `RecipeBook`,
    /// using the item name as the key.
    items: HashMap<String, Item>,
}

impl RecipeBook {
    /// Returns a new `RecipeBook` with the given name.
    pub fn new(name: String) -> RecipeBook {
        RecipeBook {
            name: name,
            items: HashMap::new(),
        }
    }

    /// Returns the relative path to a `RecipeBook` with the given `name`. (Not the filename)
    ///
    /// Example:
    ///
    /// ```
    /// path("example_book")
    /// ```
    /// returns `./recipebooks/cool_book.json`
    pub fn path(name: &String) -> String {
        format!("./recipebooks/{}.json", name)
    }

    /// Loads and returns the `RecipeBook` with the given `name`. (Not the filename)
    ///
    /// Location determined by `RecipeBook::path()`
    pub fn load(name: String) -> Result<RecipeBook, anyhow::Error> {
        let path = RecipeBook::path(&name);
        let data = load_file_to_string(&path)?;
        Ok(serde_json::from_str(&data)?)
    }

    /// Saves `self` to a file on disk. Location determined by `RecipeBook::path()`
    pub fn save(&self) -> Result<(), anyhow::Error> {
        let path = RecipeBook::path(&self.name);
        let data = serde_json::to_string_pretty(&self)?;
        std::fs::create_dir_all("recipebooks/")?;
        let mut file = OpenOptions::new().write(true).create(true).open(&path)?;
        file.write_all(&data.as_bytes())?;
        Ok(())
    }
}

/// Loads and returns the given file path as a `&str`.
pub fn load_file_to_string(path: &str) -> Result<String, anyhow::Error> {
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);
    let mut bytes = Vec::new();
    reader.read_to_end(&mut bytes)?;
    Ok(String::from_utf8(bytes)?)
}
