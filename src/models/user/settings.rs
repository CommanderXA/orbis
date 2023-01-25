use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct UserSettings {
    pub language: String,
    pub theme: String,
}
