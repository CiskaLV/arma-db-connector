// use std::thread;

use sqlx::{Pool, Postgres};
use arma_rs::{Group, Context};
use crate::config::Config;
use crate::{postgres::get_pool};

pub struct Database {
    rt: tokio::runtime::Runtime,
    pool: Pool<Postgres>,
}

pub fn group() -> Group {
    Group::new()
        .command("online", is_online)
        .command("query", query)
        .command("print", print_cfg)
        .state(Database::new("postgres"))
}

impl Database {
    pub fn new(db: &str) -> Self {
        let pool = handle(db);
        let rt = tokio::runtime::Runtime::new().unwrap();

        Self {
            pool,
            rt
        }
    }
}// "ext" callExtension ["test:print", []]

fn print_cfg(ctx: Context) -> String {
    let cfg = ctx.global().state().get::<Config>();
    format!("{:?}", &cfg)
}

fn is_online(ctx: Context) -> String {
    let db = ctx.group().unwrap().state().get::<Database>();
    let closed = db.pool.is_closed();
    format!("Database is online: {}", !closed)
}

fn query(ctx: Context) -> String {
    let db = ctx.group().unwrap().state().get::<Database>();
    db.rt.block_on(async move {
        let query = sqlx::query("SELECT 1");
        let result = query.execute(&db.pool).await;
        match result {
            Ok(_) => "Query executed".to_string(),
            Err(e) => format!("Error: {}", e)
        }
    })
}

pub fn handle(database: &str) -> Pool<Postgres> {
    match database {
        "postgres" => {
            let rt = tokio::runtime::Runtime::new().unwrap();
            
            rt.block_on(async move {
                println!("Connecting to database...");
                let pool = get_pool().await.unwrap();
                println!("Connected to database: {}", pool.size());
                pool
            })
        },
        _ => {
            unimplemented!("Database not supported")
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::init;

    #[test]
    fn database_new() {
        // let db = Database::new("postgres");
        // assert_eq!(db.pool.is_closed(), false);

        let extension = init().testing();
        let (output, _) = unsafe { extension.call("test:print", None) }; //Some(vec!["postgres".to_string()]))
        assert_eq!(output, "Connected to database: 8");
    }
}
