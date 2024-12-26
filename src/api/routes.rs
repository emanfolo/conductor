use std::sync::Arc;

use axum::{Router, routing::post};

use crate::{executor::task_executor::TaskExecutor, tasks::prime_calculator::handler::create_prime_task}; 

pub fn prime_routes(task_executor: Arc<TaskExecutor>) -> Router {
    Router::new()
        .route("/create", post(create_prime_task)
        .with_state(task_executor)
    )
}

