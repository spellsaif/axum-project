use axum::{Router, routing::get};

use crate::handlers;


pub fn routes() -> Router {
    Router::new()
    .route("/", get(handlers::hello_handler))
}