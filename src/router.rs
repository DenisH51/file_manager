use crate::handlers;
use axum::{
    Router,
    routing::get
};

use sqlx::SqlitePool;




pub fn create_router(database: SqlitePool) -> Router {

    Router::new()
        .route("/", get(handlers::pages::home))
        .with_state(database)

}
