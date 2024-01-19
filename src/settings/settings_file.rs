use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Serialize, Deserialize, Debug)]
pub struct SettingsFileMeta {
    pub version: u32,
}

// Structure for the JSON representation of settings.
// This will mirror exactly with what's on the filesystem,
// whereas the more public Settings struct wraps this file
// but converts types to be easier to work with at runtime.
#[derive(Serialize, Deserialize, Debug)]
pub struct SettingsFile {
    pub meta: SettingsFileMeta,
    pub name: String,
    pub species: String,
}

impl SettingsFile {
    pub fn new() -> Self {
        Self {
            meta: SettingsFileMeta::latest(),
            name: "Jack".to_owned(),
            species: "Wolf".to_owned(),
        }
    }

    pub fn load(filename: &str) -> Result<Self, String> {
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

    pub fn save(&self, filename: &str) -> Result<(), String> {
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

impl SettingsFileMeta {
    fn latest() -> Self {
        Self { version: 1 }
    }
}
