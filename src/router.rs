use crate::handlers;

use axum::{
    Router,
    routing::{get, post},
};

use sqlx::SqlitePool;
use tower_http::services::ServeDir;

pub fn create_router(database: SqlitePool) -> Router {
    Router::new()
        // Public pages
        .route("/", get(handlers::pages::public::home))
        .route("/login", get(handlers::pages::public::login_page))
        .route("/register", get(handlers::pages::public::registrate_page))

        // API
        .route("/api/auth/login", post(handlers::api::auth::login))
        .route("/api/auth/register", post(handlers::api::auth::register))
        .route("/api/auth/logout", post(handlers::api::auth::logout))

        // Authenticated pages
        .route("/auth/home", get(handlers::pages::auth::home::auth_home))

        // Static files
        .nest_service("/static", ServeDir::new("static"))

        .with_state(database)
}