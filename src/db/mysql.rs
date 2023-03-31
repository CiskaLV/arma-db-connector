use mysql::{Pool, prelude::Queryable, Row};
use arma_rs::{Value, IntoArma};
use crate::config::DbConfig;

#[derive(Debug)]
pub struct MySQL {
    pub pool: Pool,
}

impl MySQL {
    pub async fn new(db_config: &DbConfig) -> Self {
        let conn = format!("mysql://{}:{}@{}:{}/{}", db_config.username, db_config.password, db_config.host, db_config.port, db_config.database);
        // TODO mnight have to add more options here.
        // and mybe reduce connection pool size.
        let pool = Pool::new(conn.to_owned().as_str()).unwrap();

        Self { pool }
    }

    pub async fn query (&self, query: &str) -> Value {
        let mut conn = self.pool.get_conn().unwrap();
        // let stmt = conn.prep(query).unwrap();
        // TODO look at doing statements instead of just queries.
        let rows: Vec<Row> = conn.query(query).unwrap();
        let mut result: Vec<Value> = vec![];
        for row in rows {
            let mut row_result: Vec<Value> = vec![];
            for col in row.columns_ref() {
                let col_value = &row[col.name_str().as_ref()];

                match col_value {
                    mysql::Value::NULL => row_result.push(IntoArma::to_arma(&"NULL")),
                    mysql::Value::Bytes(bytes) => row_result.push(IntoArma::to_arma(&String::from_utf8(bytes.to_vec()).unwrap())),
                    //TODO might have to add more types in here but atm moment most things are just Bytes or NULL.
                    _ => unimplemented!(),
                }
            }
            result.push(IntoArma::to_arma(&row_result));
        }  
        let arma = IntoArma::to_arma(&result);
        // println!("{:?}", arma);
        arma
    }
}