use sqlx::{
    sqlite::SqlitePoolOptions,
    SqlitePool,
};

use dotenvy::dotenv;
use std::env;


pub async fn connect_database() -> SqlitePool{


    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL not found");


    let database = SqlitePoolOptions::new()
        .connect(&database_url)
        .await
        .expect("Database connection failed");

    sqlx::migrate!("./migrations/sqlite")
        .run(&database)
        .await
        .expect("Database migration failed");


    database
}