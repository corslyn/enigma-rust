use std::fs;
use std::path::Path;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    misc: Misc,
    reflectors: Reflectors,
    rotors: Rotors,
    notches: Notches,
}
#[derive(Debug, Deserialize)]
struct Misc {
    alphabet: String,
}

#[derive(Debug, Deserialize)]
struct Reflectors {
    A: String,
    B: String,
    C: String,
}

#[derive(Debug, Deserialize)]
struct Rotors {
    I: String,
    II: String,
    III: String,
    IV: String,
    V: String,
}

#[derive(Debug, Deserialize)]
struct Notches {
    I: String,
    II: String,
    III: String,
    IV: String,
    V: String,
}

pub fn load_config() -> Config {
    let config_path = "config.toml";
    let content = fs::read_to_string(config_path).unwrap();

    let config: Config = toml::from_str(&content).unwrap();
    return config;
}
