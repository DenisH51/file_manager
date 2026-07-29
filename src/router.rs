use crate::handlers;
use axum::{
    Router, routing::{get, post}
};

use sqlx::SqlitePool;



pub fn create_router(database: SqlitePool) -> Router {

    Router::new()
        //pages
        .route("/", get(handlers::pages::public::home))
        .route("/login", get(handlers::pages::public::login_page))
        .route("/registrate", get(handlers::pages::public::registrate_page))



        //api
        .route("/api/auth/login", post(handlers::api::auth::login))
        .route("/api/auth/registrate", post(handlers::api::auth::registrate))
        .route("/api/auth/logout", post(handlers::api::auth::logout))



        .with_state(database)
}
