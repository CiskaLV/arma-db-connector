use uuid::Uuid;
use arma_rs::{Group, Value, IntoArma};
use crate::{DATABASE, RUNTIME, LOCKED, CONFIG, Database};
use crate::config::DbKind;

pub mod postgres;
pub mod mysql;

pub fn group() -> Group {
    Group::new()
        .command("init", init_db)
        .command("lock", lock_db)
        .command("query", query_db)
        .command("update", query_db_drop)
}

fn init_db(db_name: String) -> Result<Uuid, String> {
    RUNTIME.block_on(async move {
        let locked = *LOCKED.read().await;
        if locked == true {
            error!("Database is locked");
            return Err("Database is locked".to_string())
        };

        let cfg = CONFIG.read().await;
        let Some(db_cfg) = cfg.database.iter().find(|db| db.name == db_name) else {
            error!("Database not found");
            return  Err("Database not found".to_string())
        };

        let uuid = Uuid::new_v4();

        match db_cfg.kind {
            DbKind::Postgres => {
                let postgres = postgres::Pg::new(db_cfg).await;
                let pool = postgres.pool;

                let mut db_store = DATABASE.write().await;
                db_store.insert(uuid, Database::Postgres(postgres::Pg { pool }));
            },
            DbKind::Mysql | DbKind::MariaDB => {
                let mysql = mysql::MySQL::new(db_cfg).await;
                let pool = mysql.pool;

                let mut db_store = DATABASE.write().await;
                db_store.insert(uuid, Database::MYSQL(mysql::MySQL { pool }));
            },
        }
        Ok(uuid)
    })
}

fn lock_db() -> String {
    RUNTIME.block_on(async move {
        *LOCKED.write().await = true;
    });
    info!("Database locked");
    "locked".to_string()
}

fn query_db(db_uuid: String, query: String, params: Vec<String>) -> Result<Value, Value> {
    RUNTIME.block_on(async move {
        let db_store = DATABASE.read().await;
        let Some(db) = db_store.get(&Uuid::parse_str(db_uuid.as_str()).unwrap()) else {
            return Err(IntoArma::to_arma(&"Database not found".to_string()));
        };

        if CONFIG.read().await.config.log_query.unwrap() == true {
            info!("Query: {}", query);
        }

        match db {
            Database::Postgres(db) => {
                let ret = db.query(query.as_str(), params).await?;
                Ok(ret)
            },
            Database::MYSQL(db) => {
                let ret = db.query(query.as_str(), params).await?;
                Ok(ret)
            },
        }
    })
}

fn query_db_drop(db_uuid: String, query: String, params: Vec<String>) -> Result<(), String> {
    RUNTIME.block_on(async move {
        let db_store = DATABASE.read().await;
        let Some(db) = db_store.get(&Uuid::parse_str(db_uuid.as_str()).unwrap()) else {
            return Err("Database not found".to_string());
        };

        match db {
            Database::Postgres(db) => {
                db.query_drop(query.as_str(), params).await?;
                Ok(())
            },
            Database::MYSQL(db) => {
                db.query_drop(query.as_str(), params).await?;
                Ok(())
            },
        }
    })
}
