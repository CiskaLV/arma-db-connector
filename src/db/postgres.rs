use sqlx::{Pool, Postgres, postgres::{PgPoolOptions, PgRow}, Error, Row};
use crate::config::DbConfig;
use arma_rs::{Value, IntoArma};

#[derive(Debug)]
pub struct Pg {
    pub pool: Pool<Postgres>,
}

impl Pg {
    pub async fn new(db_cfg: &DbConfig) -> Result<Pg, Error> {
        let mut conn: String = "postgres://".to_owned();
            conn.push_str(&db_cfg.username);
            conn.push_str(":");
            conn.push_str(&db_cfg.password);
            conn.push_str("@");
            conn.push_str(&db_cfg.host);
            conn.push_str(":");
            conn.push_str(&db_cfg.port.to_string());    
            conn.push_str("/");
            conn.push_str(&db_cfg.database);

        let pool =  PgPoolOptions::new()
            .min_connections(1)
            .max_connections(5)
            .connect(conn.to_owned().as_str())
            .await
            .expect("Failed to create Postgres connection pool");

        Ok(Self { pool })
    }

    pub async fn query(&self, query: &str) -> Value {
        let mut conn = self.pool.acquire().await.expect("Failed to acquire connection");
        let result = sqlx::query(query).fetch_all(&mut conn).await.expect("Failed to fetch all rows");

        let mut ret = Vec::new();
        for row in result {
            for i in 0..row.len() {
                let val = row.get(i);
                ret.push(val);
            }
        }


        // arma_rs::Value::Array(ret)
    }
}
