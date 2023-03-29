// #[macro_use]
// extern crate log;

// mod logger;

use std::collections::HashMap;
use uuid::Uuid;

use arma_rs::{arma, Extension};
use tokio::sync::RwLock;

mod db;
pub mod config;


#[derive(Debug)]
pub enum Database {
    Postgres(db::postgres::Pg),
}

lazy_static::lazy_static! {
    static ref RUNTIME: tokio::runtime::Runtime = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .expect("Failed to create Tokio runtime");
    pub static ref DATABASE: RwLock<HashMap<Uuid, Database>> = RwLock::new(HashMap::new());
    pub static ref LOCKED: RwLock<bool> = RwLock::new(false);
    static ref CONFIG: RwLock<config::Config> = RwLock::new(config::Config::new());
}

#[arma]
fn init() -> Extension {
    let ext = Extension::build()
        .version("0.1.0".to_owned())
        .group("db", db::group())
        .command("test",test)
        .finish();

        // logger::init(ext.context());
  
    ext
}

fn test() -> arma_rs::Value {
    let vev = vec![vec![1,2,3],vec![1,2,3],vec![1,2,]];
    let arma_format = arma_rs::IntoArma::to_arma(&vev);
    println!("{}", arma_format);
    arma_format
}

#[cfg(test)]
mod tests {
    use super::{init};
    use uuid::Uuid;

    #[test]
    fn test_vec() {
        let extension = init().testing();
        let (output, _) = unsafe { extension.call("test", None) };
        println!("{}", output);
        // assert_eq!(output, Uuid::nil().to_string());
    }

    #[test]
    fn test_test() {
        let extension = init().testing();
        let (output, _) = unsafe { extension.call("db:init", Some(vec!["pg".to_string()])) };
        let (o, err) = unsafe { extension.call("db:query", Some(vec![output, "SELECT 1;".to_string()])) };

        println!("Result: {},  Error: {}", o, err);
        // assert_eq!(output, Uuid::nil().to_string());
    }
}