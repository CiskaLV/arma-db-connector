// use tokio;

// #[tokio::main]
// async fn main() {
//     println!("Hello, world!");
//     tokio::spawn(async {
//         println!("Hello from the spawned future!");
//     });
// }

use std::thread;
use std::sync::{Arc, Mutex};
use sqlx::postgres::{PgPoolOptions};


fn main() {
    // Create a SQLX connection pool with 5 connections
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://username:password@localhost/database")
        .await
        .unwrap();

    // Wrap the pool in an Arc and a Mutex to make it thread-safe
    let pool = Arc::new(Mutex::new(pool));

    // Create a thread with a unique ID and pass the pool as a parameter
    let thread_pool = Arc::clone(&pool);
    let thread_id = 1;
    let thread_handle = thread::spawn(move || {
        println!("Thread {} started", thread_id);

        // Get a connection from the pool and execute a query
        let conn = thread_pool.lock().unwrap().acquire().await.unwrap();
        let rows = sqlx::query("SELECT * FROM my_table").fetch_all(&conn).await.unwrap();
        println!("Thread {} fetched {} rows", thread_id, rows.len());

        println!("Thread {} finished", thread_id);
    });

    // Wait for the thread to finish and join it
    thread_handle.join().unwrap();

    // Now we can call the thread to execute commands using the pool from the main thread
    let main_conn = pool.lock().unwrap().acquire().await.unwrap();
    let main_rows = sqlx::query("SELECT * FROM my_table").fetch_all(&main_conn).await.unwrap();
    println!("Main thread fetched {} rows", main_rows.len());
}
