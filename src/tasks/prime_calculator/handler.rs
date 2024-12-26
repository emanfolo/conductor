use std::sync::Arc;

use axum::{extract::State, Json, http::StatusCode};
use tokio::sync::mpsc;
use uuid::Uuid;

use crate::{executor::{task_executor::TaskExecutor, CompletedMetrics, ProgressMetrics, TaskResponse, TaskState}, types::prime_calculation::{PrimeCalculationProgressMetrics, PrimeCalculationRequest}};

use super::calculator::PrimeCalculator;

#[axum::debug_handler]
pub async fn create_prime_task(
    State(task_executor): State<Arc<TaskExecutor>>,
    Json(input): Json<PrimeCalculationRequest>,
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
                Ok(metrics) => {
                    if let Err(e) = executor.store_result(task_id, CompletedMetrics::PrimeCalculationMetrics(metrics)).await {
                        eprintln!("Failed to store result: {:?}", e);
                    }
                }
                Err(e) => {
                        let _ = executor.store_failure(task_id).await;
                        eprintln!("Failed to execute task with error: {:?}", e);
                }
            }
        }
     });

    Ok(Json(TaskResponse {
        task_id: task_id.to_string(),
        state: TaskState::Running(
            ProgressMetrics::PrimeCalculationMetrics(PrimeCalculationProgressMetrics {
                current_number: 0,
                found_primes: 0,
                percentage_complete: 0.0,
                current_memory_usage: 0,
                elapsed_time_ms: 0,
            })
        ),
    }))
}