use axum::{routing::get, Router, Json};
use serde::Serialize;
use std::net::SocketAddr;
use crate::types::prime_calculation::{PrimeCalculationRequest, PrimeCalculationMetrics, PrimeProgress};

pub mod types;

#[derive(Serialize)]
struct HelloResponse {
    message: String,
}


fn say_hello() -> String {
    "Hello, world!".to_string()
}

async fn health() -> Json<HelloResponse> {
    Json(HelloResponse {
        message: say_hello(),
    })
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/health", get(health));

    let addr = SocketAddr::from(([127, 0, 0, 1], 5001));

    println!("Server running on {}", addr);
    axum::serve(
        tokio::net::TcpListener::bind(addr)
            .await
            .unwrap(),
        app
    )
    .await
    .unwrap();
        
}