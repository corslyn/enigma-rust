use serde::Deserialize;
use std::{collections::HashMap, fs};

#[derive(Debug, Deserialize)]
pub struct Config {
    pub misc: Misc,
    pub reflectors: HashMap<String, String>,
    pub rotors: HashMap<String, String>,
    pub notches: HashMap<String, char>, // Each notch is a single character
}
#[derive(Debug, Deserialize)]
pub struct Misc {
    pub alphabet: String,
}

#[derive(Debug, Deserialize)]
pub struct Reflectors {
    pub A: String,
    pub B: String,
    pub C: String,
}

#[derive(Debug, Deserialize)]
pub struct Rotors {
    pub I: String,
    pub II: String,
    pub III: String,
    pub IV: String,
    pub V: String,
}

#[derive(Debug, Deserialize)]
pub struct Notches {
    pub I: String,
    pub II: String,
    pub III: String,
    pub IV: String,
    pub V: String,
}

pub fn load_config() -> Config {
    let config_path = "config.toml";
    let content = fs::read_to_string(config_path).unwrap();

    let config: Config = toml::from_str(&content).unwrap();
    return config;
}
