use crate::fursona::Fursona;
use std::path::Path;

mod settings_file;

use settings_file::{CurrentSettingsFile, LoadSettingsResult};

pub struct Settings {
    pub fursona: Vec<Fursona>,
}

impl Settings {
    fn from_settings_file(file: &CurrentSettingsFile) -> Self {
        Self {
            fursona: file
                .fursona
                .iter()
                .map(|fursona| Fursona {
                    name: fursona.name.to_owned(),
                })
                .collect(),
        }
    }

    pub fn load_or_create(filename: &str) -> Self {
        // Attempt to load the file if it already exists
        if Path::new(filename).exists() {
            println!("Attempting to load {filename}");
            match CurrentSettingsFile::load(filename) {
                LoadSettingsResult::Success { file, did_migrate } => {
                    println!("Successfully loaded settings file");
                    let parsed = Settings::from_settings_file(&file);

                    if did_migrate {
                        println!("Saving settings file to migrate to latest version");
                        if let Err(msg) = file.save(filename) {
                            println!("Error saving migrated file: {msg}");
                        }
                    }

                    return parsed;
                }
                LoadSettingsResult::Error(e) => {
                    println!("Load of file {filename} failed: {e}");
                }
            }
        } else {
            println!("File {filename} doesn't exist");
        }

        // The file didn't load, so we'll create a new file from scratch
        let created_file = CurrentSettingsFile::new();
        let created = Settings::from_settings_file(&created_file);

        // Let's serialize this file to the filesystem
        println!("Saving new settings file to {filename}");
        if let Err(msg) = created_file.save(filename) {
            println!("Error saving new file: {msg}");
        }

        // Return this newly created file
        created
    }
}
