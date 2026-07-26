use crate::handlers;
use axum::{
    Router,
    routing::get
};

use crate::database::Database;




pub fn create_router(database: Database) -> Router {

    Router::new()
        .route("/", get(handlers::pages::home))
        


    

}
