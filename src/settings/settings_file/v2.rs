use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct V2SettingsFileFursona {
    pub name: String,
}

#[derive(Serialize, Deserialize)]
pub struct V2SettingsFile {
    pub fursona: Vec<V2SettingsFileFursona>,
}

impl V2SettingsFile {
    pub fn new() -> Self {
        Self {
            fursona: Vec::new(),
        }
    }
}
