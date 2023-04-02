use toml::from_str;
use std::fs;
use std::process::exit;
use serde_derive::Deserialize;


#[derive(Deserialize, Debug, Clone)]
pub struct Config {
    pub config: Settings,
    pub database: Vec<DbConfig>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Settings {
    pub log_query: Option<bool>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct DbConfig {
    pub name: String,
    pub kind: DbKind,
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
    pub database: String,
}

#[derive(Deserialize, Debug, Clone)]
pub enum DbKind {
    #[serde(rename = "postgres")]
    Postgres,
    #[serde(rename = "mysql")]
    Mysql,
    #[serde(rename = "mariadb")]
    MariaDB,
}

impl Config {
    pub fn new() -> Self {
    let filename = "./@daveDB/config.toml";
    // let filename = "D:\\Reppos\\arma-db-connector\\daveDB\\config.toml";
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
