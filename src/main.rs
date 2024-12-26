use axum::{routing::{get, post}, Json, Router};
use executor::task_executor::TaskExecutor;
use serde::Serialize;
use streaming::task_stream::stream_all_tasks;
use tasks::prime_calculator::handler::create_prime_task;
use std::{net::SocketAddr, sync::Arc};

pub mod types;
pub mod tasks;
pub mod executor;
pub mod api;
pub mod streaming;

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
    let task_executor = Arc::new(TaskExecutor::new());


    let app = Router::new()
        .route("/health", get(health))
        .route("/api/prime", post(create_prime_task))
        .route("/api/stream", get(stream_all_tasks))
        .with_state(task_executor);

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