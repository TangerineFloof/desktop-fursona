use crate::fursona::Fursona;
use std::path::Path;

mod settings_file;

pub struct Settings {
    pub fursona: Vec<Fursona>,
}

impl Settings {
    fn from_settings_file(file: settings_file::SettingsFile) -> Self {
        Self {
            fursona: file
                .fursona
                .into_iter()
                .map(|fursona| fursona.to_runtime())
                .collect(),
        }
    }

    pub fn load_or_create(filename: &str) -> Self {
        // Attempt to load the file if it already exists
        if Path::new(filename).exists() {
            println!("Attempting to load {filename}");
            match settings_file::SettingsFile::load(filename) {
                Ok(file) => {
                    println!("Successfully loaded settings file");
                    return Settings::from_settings_file(file);
                }
                Err(e) => {
                    println!("Load of file {filename} failed: {e}");
                }
            }
        } else {
            println!("File {filename} doesn't exist");
        }

        // The file didn't load, so we'll create a new file from scratch
        let created = settings_file::SettingsFile::new();

        // Let's serialize this file to the filesystem
        println!("Saving new settings file to {filename}");
        if let Err(msg) = created.save(filename) {
            println!("Error saving new file: {msg}");
        }

        // Return this newly created file
        Settings::from_settings_file(created)
    }
}
