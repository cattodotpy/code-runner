use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone)]
pub struct Config {
    pub port: Option<u16>,
    pub address: Option<String>,
    pub code_dir: String,
    pub languages: HashMap<String, Language>,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            port: Some(8080),
            address: Some("0.0.0.0".to_string()),
            code_dir: "code".to_string(),
            languages: HashMap::new(),
        }
    }
}

#[derive(Deserialize, Serialize, Clone)]
pub struct Language {
    pub compile: Option<Vec<String>>,
    pub run: Vec<String>,
    pub extension: String,
}
