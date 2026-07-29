mod router;
mod handlers;
mod database;
mod services;
mod templates;
mod models;
mod errors;

use tokio::net::TcpListener;





#[tokio::main]
async fn main(){

    let server_address: String = "127.0.0.1:8080".to_string();

    let database = database::connect_database().await;

    let router = router::create_router(database);


    let listener = TcpListener::bind(server_address)
        .await
        .expect("Unable to connect to the server");

    println!("Server started");

    axum::serve(listener, router)
        .await
        .unwrap();
}