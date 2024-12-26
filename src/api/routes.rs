use axum::{Router, routing::{post, get}}; 

pub fn prime_routes() -> Router {
    Router::new()
        .route("/create", post(create_prime_task)
    )
}

