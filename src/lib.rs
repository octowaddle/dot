extern crate clap;
extern crate dirs;
extern crate serde;
extern crate toml;

pub mod config;
pub mod options;
pub mod set;

use config::Config;
use options::{Action, Filter, Options};
use std::{error::Error, path::Path};

pub fn run(options: &Options) -> Result<(), Box<dyn Error>> {
    let config = Config::from_file(Path::new("dot.toml"))?;
    let root = Path::new("");

    if let Some(action) = &options.action {
        match action {
            Action::Apply => {
                if let Some(filter) = &options.filter {
                    match filter {
                        Filter::All => {
                            for set in config.set {
                                set.apply(&root)?;
                            }
                        }
                        Filter::Match(criteria) => {
                            for set in config.set.iter().filter(|set| {
                                criteria.names.contains(&set.name)
                                    || criteria.groups.contains(&set.group)
                            }) {
                                set.apply(&root)?;
                            }
                        }
                    }
                }
            }
            Action::Save => {
                if let Some(filter) = &options.filter {
                    match filter {
                        Filter::All => {
                            for set in config.set {
                                set.save(&root)?;
                            }
                        }
                        Filter::Match(criteria) => {
                            for set in config.set.iter().filter(|set| {
                                criteria.names.contains(&set.name)
                                    || criteria.groups.contains(&set.group)
                            }) {
                                set.save(&root)?;
                            }
                        }
                    }
                }
            }
        }
    }

    Ok(())
}
