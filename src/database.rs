use sqlx::{
    postgres::PgPoolOptions,
    PgPool,
};

use dotenvy::dotenv;
use std::env;


pub async fn connect_database() -> PgPool{


    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL not found");


    let database = PgPoolOptions::new()
        .connect(&database_url)
        .await
        .expect("Database connection failed");

    sqlx::migrate!("./migrations/postgre")
        .run(&database)
        .await
        .expect("Database migration failed");



    database
}