use sqlx::{Pool, Postgres, postgres::{PgPoolOptions, PgRow}, Error, Row, ValueRef, Column};
use crate::config::DbConfig;
use arma_rs::{Value, IntoArma};

use std::collections::HashMap;

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

    pub async fn query(&self, query: &str) -> Vec<String> {
        let mut conn = self.pool.acquire().await.expect("Failed to acquire connection");
        let result = sqlx::query(query).fetch_one(&mut conn).await.expect("Failed to fetch all rows");

        let row = row_to_vec(result);
        row
    }
}


fn row_to_vec(row: PgRow) -> Vec<String> {
    let mut result = Vec::<String>::new();
    for col in row.columns() {
        let value = row.try_get_raw(col.ordinal()).unwrap();
        // let s = value.as_str().unwrap().to_string();
        let s =  value.format();
        
        // let value = match value.is_null() {
        //     true => "NULL".as_bytes(),
        //     false => value.as_bytes().unwrap(),
        // };
        // println!("{:?}", value);

        // let s = std::str::from_utf8(value).unwrap().to_string();
        println!("{:?}", s);
        result.push(s);
    }
    result
}