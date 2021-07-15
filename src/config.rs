use crate::set::Set;
use serde::Deserialize;
use std::{error::Error, fs, path::Path};

#[derive(Debug, PartialEq, Eq, Deserialize)]
pub struct Config {
    pub set: Vec<Set>,
}

impl Config {
    pub fn from_file(path: &Path) -> Result<Self, Box<dyn Error>> {
        let contents = fs::read_to_string(path)?;
        let config = toml::from_str(&contents)?;
        Ok(config)
    }
}
