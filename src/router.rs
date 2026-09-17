use crate::{
    handlers,
    middleware,
};

use axum::{
    middleware::from_fn_with_state,
    routing::{get, post},
    Router,
};

use sqlx::SqlitePool;
use tower_http::services::ServeDir;



pub fn create_router(database: SqlitePool) -> Router {

    //router only for authorised users 
    let auth_router = Router::new()
        
        //protected routes
        .route(
            "/home",
            get(handlers::pages::auth::home::home),
        )



        //Before executing any route inside this Router, execute middleware
        //Middleware check session Ok -> give access !Ok -> redirect /login
        .route_layer(
            from_fn_with_state(
                database.clone(),
                middleware::auth::require_auth,
            )
        );

    Router::new()

        // Public pages
        .route("/", get(handlers::pages::public::home))
        .route("/login", get(handlers::pages::public::login_page))
        .route("/register", get(handlers::pages::public::register_page))

        // API
        .route("/api/auth/login", post(handlers::api::auth::login))
        .route("/api/auth/register", post(handlers::api::auth::register))
        .route("/api/auth/logout", post(handlers::api::auth::logout))

        // Protected
        .nest("/auth", auth_router)

        // Static
        .nest_service("/static", ServeDir::new("static"))

        .with_state(database)
}