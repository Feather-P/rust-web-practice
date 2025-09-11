use axum::{Router, routing::get};
use crate::handlers::{helloworld};

pub fn api_route () -> Router {
    Router::new()
        .nest("/api", merge_api_routes())
}

fn merge_api_routes() -> Router {
    Router::new()
    .merge(helloworld_route())
}

fn helloworld_route() -> Router {
    Router::new()
        .route("/helloworld", get(helloworld::helloworld))
}