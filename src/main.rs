mod router;
mod handlers;
mod middleware;
mod database;
mod services;
mod templates;
mod models;
mod responses;
mod extractor;

use tracing_subscriber;
use std::env;
use tokio::net::TcpListener;


#[tokio::main]
async fn main(){
    tracing_subscriber::fmt::init();

    dotenvy::dotenv().expect("Failed to load .env file");

    let server_address = env::var("SERVER_ADDRESS")
        .expect("SERVER_ADDRESS is not set");

    let database = database::connect_database().await;

    let router = router::create_router(database);


    let listener = TcpListener::bind(server_address)
        .await
        .expect("Unable to connect to the server");

    tracing::info!("Server started");
    
    axum::serve(listener, router)
        .await
        .unwrap();
}