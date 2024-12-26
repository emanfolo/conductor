use std::sync::Arc;

use axum::{extract::State, Json};
use tokio::sync::mpsc;
use uuid::Uuid;

use crate::{executor::{task_executor::TaskExecutor, TaskResponse, TaskState}, types::{prime_calculation::PrimeCalculationRequest, requests::StatusCode}};

use super::calculator::PrimeCalculator;

pub async fn create_prime_task(
    Json(input): Json<PrimeCalculationRequest>,
    State(task_executor): State<Arc<TaskExecutor>>,
) -> Result<Json<TaskResponse>, StatusCode> {
    let (progress_tx, _progress_rx) = mpsc::channel(32);

    let task_id = Uuid::new_v4();

    let calculator = PrimeCalculator::new(
        input.upper_bound,
        input.batch_size.unwrap_or(10000),
        progress_tx,
    );
    
    tokio::spawn({
        let executor = task_executor.clone();
        async move {
            match calculator.calculate().await {
                Ok((primes, metrics)) => {
                    if let Err(e) = executor.store_result(task_id, primes, metrics).await {
                        eprintln!("Failed to store result: {:?}", e);
                    }
                }
                Err(e) => {
                        eprintln!("Failed to execute task with error: {:?}", e);
                }
            }
        }
     });

    Ok(Json(TaskResponse {
        task_id: task_id.to_string(),
        state: TaskState::Running,
    }))
}