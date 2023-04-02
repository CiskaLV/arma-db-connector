#[macro_use]
extern crate log;
mod logger;

use std::collections::HashMap;
use uuid::Uuid;
use arma_rs::{arma, Extension};
use tokio::sync::RwLock;

mod db;
pub mod config;


#[derive(Debug)]
pub enum Database {
    Postgres(db::postgres::Pg),
    MYSQL(db::mysql::MySQL),
}

lazy_static::lazy_static! {
    static ref RUNTIME: tokio::runtime::Runtime = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .expect("Failed to create Tokio runtime");
    pub static ref DATABASE: RwLock<HashMap<Uuid, Database>> = RwLock::new(HashMap::new());
    pub static ref LOCKED: RwLock<bool> = RwLock::new(false);
    pub static ref CONFIG: RwLock<config::Config> = RwLock::new(config::Config::new());
}

#[arma]
fn init() -> Extension {
    let ext = Extension::build()
        .version("0.1.0".to_owned())
        .group("db", db::group())
        .command("uuid", command_uuid)
        .finish();

        logger::init(ext.context());

    ext
}

fn command_uuid() -> Uuid {
    let uuid = Uuid::new_v4();
    info!("Generating new UUID v4 for Arma 3 extension call to Rust library. {}", uuid);
    uuid
}

#[cfg(test)]
mod tests {
    use super::init;
    // use uuid::Uuid;

    #[test]
    fn postgres() {
        let extension = init().testing();
        let (output, _) = unsafe { extension.call("db:init", Some(vec!["pg".to_string()])) };
        println!("{}", output);
        let (o, err) = unsafe { extension.call("db:query", Some(vec![output, "SELECT * FROM contacts LIMIT 100;".to_string()])) };
    
        println!("Result: {},  Error: {}", o, err);
        // assert_eq!(output, Uuid::nil().to_string());
    }

    #[test]
    fn mysql() {
        let extension = init().testing();
        let (output, _) = unsafe { extension.call("db:init", Some(vec!["mysql".to_string()])) };
        println!("{}", output);
        let (o, err) = unsafe { extension.call("db:query", Some(vec![output, "SELECT * FROM players LIMIT 1;".to_string(), "[]".to_string()])) };

        println!("Result: {},  Error: {}", o, err);
    }
}
