mod v1;
mod v2;

use serde::{Deserialize, Serialize};
use std::fs;

// Structure for the JSON representation of settings.
// This will mirror exactly with what's on the filesystem,
// whereas the more public Settings struct wraps this file
// but converts types to be easier to work with at runtime.
#[derive(Serialize, Deserialize)]
#[serde(tag = "version")]
enum SettingsFile {
    #[serde(rename = "1")]
    V1(v1::V1SettingsFile),
    #[serde(rename = "2")]
    V2(v2::V2SettingsFile),
}

pub type CurrentSettingsFile = v2::V2SettingsFile;

impl SettingsFile {
    fn load(filename: &str) -> Result<SettingsFile, String> {
        // Read the file from the filesystem
        let file_contents = match fs::read_to_string(filename) {
            Ok(str) => str,
            Err(e) => {
                return Err(e.to_string());
            }
        };

        // Parse the string into a JSON object
        let parsed: Result<SettingsFile, serde_json::Error> = serde_json::from_str(&file_contents);
        match parsed {
            Ok(j) => Ok(j),
            Err(e) => Err(e.to_string()),
        }
    }

    fn save(&self, filename: &str) -> Result<(), String> {
        // Convert the structure to a JSON string
        let file_contents = match serde_json::to_string_pretty(self) {
            Ok(str) => str,
            Err(msg) => {
                return Err(msg.to_string());
            }
        };

        // Write the string to the filesystem
        match fs::write(filename, file_contents) {
            Ok(_) => Ok(()),
            Err(e) => Err(e.to_string()),
        }
    }
}

pub enum LoadSettingsResult {
    Success {
        file: CurrentSettingsFile,
        did_migrate: bool,
    },
    Error(String),
}

impl CurrentSettingsFile {
    pub fn load(filename: &str) -> LoadSettingsResult {
        let mut current = match SettingsFile::load(filename) {
            Ok(file) => file,
            Err(e) => return LoadSettingsResult::Error(e),
        };

        let mut did_migrate = false;
        loop {
            current = match current {
                SettingsFile::V1(file) => {
                    println!("Migrating settings v1 -> v2");
                    did_migrate = true;
                    SettingsFile::V2(file.migrate())
                }
                SettingsFile::V2(file) => return LoadSettingsResult::Success { did_migrate, file },
            }
        }
    }

    pub fn save(self, filename: &str) -> Result<(), String> {
        SettingsFile::V2(self).save(filename)
    }
}
