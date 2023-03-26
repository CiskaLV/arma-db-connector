use sqlx::{postgres::{PgPoolOptions}, Pool, Postgres};


pub async fn get_pool() -> Result<Pool<Postgres>, sqlx::Error> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://postgres:postgres@192.168.8.10/postgres")
        .await?;

    Ok(pool)
}
