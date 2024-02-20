use super::v2::{V2SettingsFile, V2SettingsFileFursona};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct V1SettingsFile {
    pub name: String,
    pub species: String,
}

impl V1SettingsFile {
    pub fn migrate(&self) -> V2SettingsFile {
        V2SettingsFile {
            fursona: vec![V2SettingsFileFursona {
                name: self.name.to_owned(),
            }],
        }
    }
}
