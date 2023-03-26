use toml::from_str;
use std::fs;
use std::process::exit;
use serde_derive::Deserialize;


#[derive(Deserialize, Debug)]
pub struct Config {
    pub config: Settings,
    pub database: Vec<DbConfig>,
}

#[derive(Deserialize, Debug)]
pub struct Settings {
    loglevel: Option<bool>,
}

#[derive(Deserialize, Debug)]
pub struct DbConfig {
    pub name: String,
    pub kind: String,
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
    pub database: String,
}

impl Config {
    pub fn new() -> Self {
        let filename = "D:\\Reppos\\arma-db-connector\\src\\config.toml";
    let content = match fs::read_to_string(filename) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Error reading config file: {}", e);
            exit(1);
        }
    };

    let data: Config = match from_str(&content) {
        Ok(d) => d,
        Err(e) => {
            eprintln!("Error parsing config file: {}", e);
            exit(1);
        }
    };

    data
    }
}
