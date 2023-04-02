use mysql::{Pool, prelude::Queryable, Row};
use arma_rs::{Value, IntoArma};
use crate::config::DbConfig;
use chrono::NaiveDate;

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

    pub async fn query (&self, query: &str, params: Vec<String>) -> Result<Value, String> {
        let mut conn = self.pool.get_conn().unwrap();
        let stmt = conn.prep(query).unwrap();

        let rows: Vec<Row> = conn.exec(stmt, params).unwrap();
        println!("{:?}", rows);
        let mut result: Vec<Value> = vec![];
        for row in rows {
            let mut row_result: Vec<Value> = vec![];
            for col in row.columns_ref() {
                let col_value = &row[col.name_str().as_ref()];
                
                match col_value {
                    mysql::Value::NULL => row_result.push(Value::Null),
                    mysql::Value::Int(value) => row_result.push(IntoArma::to_arma(&(*value as f64))),
                    mysql::Value::Bytes(bytes) => row_result.push(IntoArma::to_arma(&String::from_utf8(bytes.to_vec()).unwrap())),
                    mysql::Value::Date(year, month, day, hour, minutes, seconds, micro) => {
                        let date = NaiveDate::from_ymd_opt(*year as i32, *month as u32, *day as u32).unwrap();
                        let time = date.and_hms_micro_opt(*hour as u32, *minutes as u32, *seconds as u32, *micro as u32).unwrap();

                        row_result.push(IntoArma::to_arma(&time))
                    },
                    mysql::Value::Float(value) => row_result.push(IntoArma::to_arma(&(*value as f64))),
                    mysql::Value::Double(value) => row_result.push(IntoArma::to_arma(&(*value as f64))),
                    _ => unimplemented!(),
                }
            }
            result.push(IntoArma::to_arma(&row_result));
        }
        let arma = IntoArma::to_arma(&result);
        // println!("{:?}", arma);
        Ok(arma)
    }

    pub async fn query_drop(&self, query: &str, params: Vec<String>) -> Result<(), String> {
        let mut conn = self.pool.get_conn().unwrap();
        let stmt = conn.prep(query).unwrap();

        let res = conn.exec_drop(stmt, params).unwrap();
        Ok(res)
    }
}