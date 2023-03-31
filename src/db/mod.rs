use uuid::Uuid;
use arma_rs::{Group, Value};
use crate::{DATABASE, RUNTIME, LOCKED, CONFIG, Database};

pub mod postgres;
pub mod mysql;

pub fn group() -> Group {
    Group::new()
        .command("init", init_db)
        .command("lock", lock_db)
        .command("query", query_db)
}

fn init_db(db_name: String) -> Uuid {
    RUNTIME.block_on(async move {
        let locked = *LOCKED.read().await;
        if locked == true {
            return Uuid::nil()
        };

        let cfg = CONFIG.read().await;
        let Some(db_cfg) = cfg.database.iter().find(|db| db.name == db_name) else {
            error!("Database not found {}", db_name);
            return Uuid::nil()
        };



        match db_cfg.kind.as_ref() {
            "postgres" => {
                // let pool = postgres::Pg::new(db_cfg).await
                //     .expect("Failed to create Postgres connection pool")
                //     .pool;
                let postgres = postgres::Pg::new(db_cfg).await;
                let pool = postgres.pool;

                let uuid = Uuid::new_v4();

                let mut db_store = DATABASE.write().await;
                db_store.insert(uuid, Database::Postgres(postgres::Pg { pool }));
                // println!("{:?}", db_store);
                // let mut db_store = DATABASE.write().await;
                // db_store.insert(uuid, Database::Postgres(postgres::Pg { pool }));
                
                uuid
            },
            "mysql" | "mariadb" => {
                let mysql = mysql::MySQL::new(db_cfg).await;
                let pool = mysql.pool;
                let uuid = Uuid::new_v4();

                let mut db_store = DATABASE.write().await;
                db_store.insert(uuid, Database::MYSQL(mysql::MySQL { pool }));

                uuid
            },
            _ => {
                error!("Database type not supported {}", db_cfg.kind);
                return Uuid::nil();
            }
        }
    })
}

fn lock_db() -> String {
    RUNTIME.block_on(async move {
        *LOCKED.write().await = true;
    });
    "locked".to_string()
}


fn query_db(db_uuid: String, query: String ) -> Value {
    RUNTIME.block_on(async move {
        let db_store = DATABASE.read().await;
        let Some(db) = db_store.get(&Uuid::parse_str(db_uuid.as_str()).unwrap()) else {
            return arma_rs::IntoArma::to_arma(&"Database not found".to_string())
        };

        info!("Querying database: {}", query);

        match db {
            Database::Postgres(db) => {
                let ret = db.query(query.as_str()).await;
                ret
            },
            Database::MYSQL(db) => {
                let ret = db.query(query.as_str()).await;
                ret
            },
            // _ => unimplemented!()
        }
    })
}
