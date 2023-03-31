use arma_rs::{Value, IntoArma};
use deadpool_postgres::{Config, Pool, Runtime};
use tokio_postgres::NoTls;
use tokio_postgres::types::Type;
use crate::config::DbConfig;

#[derive(Debug)]
pub struct Pg {
    pub pool: Pool,
}

impl Pg {
    pub async fn new(db_cfg: &DbConfig) -> Self {
        let mut cfg = Config::new();

        cfg.host = Some(db_cfg.host.to_owned());
        cfg.port = Some(db_cfg.port);
        cfg.user = Some(db_cfg.username.to_owned());
        cfg.dbname = Some(db_cfg.database.to_owned());
        cfg.password = Some(db_cfg.password.to_owned());

        let pool = cfg.create_pool(Some(Runtime::Tokio1), NoTls).unwrap();

        Self { pool }
    }

    pub async fn query(&self, query: &str) -> Value {
        let client = self.pool.get().await.unwrap();
        let stmt = client.prepare(query).await.unwrap();
        // let rows = client.query(&stmt, &[]).await.unwrap();

        let mut result: Vec<Value> = vec![];
        for row in client.query(&stmt, &[]).await.unwrap() {

            let mut vec: Vec<Value> = vec![];
            for (i,col) in row.columns().iter().enumerate() {
                match col.type_().to_owned() {
                    Type::INT2 => {
                        let value: i16 = row.try_get(i).unwrap();
                        vec.push(IntoArma::to_arma(&value));
                    },
                    Type::INT4 => {
                        let value: i32 = row.try_get(i).unwrap();
                        vec.push(IntoArma::to_arma(&value));
                    },
                    Type::FLOAT4 => {
                        let value: f32 = row.try_get(i).unwrap();
                        vec.push(IntoArma::to_arma(&value));
                    },
                    Type::FLOAT8 => {
                        let value: f64 = row.try_get(i).unwrap();
                        vec.push(IntoArma::to_arma(&value));
                    },
                    Type::TEXT => {
                        let value: String = row.try_get(i).unwrap();
                        vec.push(IntoArma::to_arma(&value));
                    },
                    Type::BOOL => {
                        let value: bool = row.try_get(i).unwrap();
                        vec.push(IntoArma::to_arma(&value));
                    },
                    Type::VARCHAR => {
                        let value: String = row.try_get(i).unwrap();
                        vec.push(IntoArma::to_arma(&value));
                    },
                    _ => unimplemented!()
                }
            }
            result.push(IntoArma::to_arma(&vec));
        }
        // println!("{:?}", result);
        IntoArma::to_arma(&result)
    }
}