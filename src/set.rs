use serde::Deserialize;
use std::{
    error::Error,
    fs,
    path::{Path, PathBuf},
};

#[derive(Debug, PartialEq, Eq, Deserialize)]
pub struct Set {
    pub name: String,
    pub group: String,
    files: Vec<PathBuf>,
}

impl Set {
    pub fn save(&self, root: &Path) -> Result<(), Box<dyn Error>> {
        for file in self.files.iter() {
            let source = dirs::home_dir().unwrap().join(file);
            let destination = root.join(file);
            fs::create_dir_all(&destination.parent().unwrap())?;
            fs::copy(&source, &destination)?;
        }
        Ok(())
    }

    pub fn apply(&self, root: &Path) -> Result<(), Box<dyn Error>> {
        for file in self.files.iter() {
            let source = root.join(file);
            let destination = dirs::home_dir().unwrap().join(file);
            fs::create_dir_all(&destination.parent().unwrap())?;
            fs::copy(&source, &destination)?;
        }
        Ok(())
    }
}
