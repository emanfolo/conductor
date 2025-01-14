use std::sync::Arc;

use axum::{extract::State, Json, http::StatusCode};
use tokio::sync::mpsc;
use uuid::Uuid;

use crate::{executor::{task_executor::TaskExecutor, CompletedMetrics, ProgressMetrics, TaskCompletion, TaskProgress, TaskResponse, TaskState}, types::prime_calculation::{PrimeCalculationProgressMetrics, PrimeCalculationRequest}};

use super::calculator::PrimeCalculator;

#[axum::debug_handler]
pub async fn create_prime_task(
    State(task_executor): State<Arc<TaskExecutor>>,
    Json(input): Json<PrimeCalculationRequest>,
) -> Result<Json<TaskResponse>, StatusCode> {
    let (progress_tx, progress_rx) = mpsc::channel(32);

    let task_id = Uuid::new_v4();

    let calculator = PrimeCalculator::new(
        input.upper_bound,
        input.batch_size.unwrap_or(10000),
        progress_tx,
    );

    let initial_metrics = TaskProgress {
        metrics: ProgressMetrics::PrimeCalculationMetrics(PrimeCalculationProgressMetrics {
            current_number: 0,
            found_primes: 0,
            percentage_complete: 0.0,
            current_memory_usage: 0,
            elapsed_time_ms: 0,
        }),
        visualisation: None,
        timestamp: 0,
    };

    let _ = task_executor.register_task(task_id, initial_metrics.clone()).await;
    
    tokio::spawn({
        let executor = task_executor.clone();
        let mut progress_rx = progress_rx;
        async move {
            // Progress updates
            let progress_handler = tokio::spawn({
                let executor = executor.clone();
                async move {
                    while let Some(progress) = progress_rx.recv().await {
                        let progress_update = TaskProgress {
                            metrics: ProgressMetrics::PrimeCalculationMetrics(progress),
                            visualisation: None,  // or generate visualization
                            timestamp: std::time::SystemTime::now()
                                .duration_since(std::time::UNIX_EPOCH)
                                .unwrap()
                                .as_secs(),
                        };
                        let _ = executor.update_progress(task_id, progress_update).await;
                    }
                }
            });

            match calculator.calculate().await {
                Ok(metrics) => {
                    let completion = TaskCompletion {
                        metrics: CompletedMetrics::PrimeCalculationMetrics(metrics),
                        visualisation: Vec::new(), // or collect visualization frames if you have them
                        timestamp: std::time::SystemTime::now()
                            .duration_since(std::time::UNIX_EPOCH)
                            .unwrap()
                            .as_secs(),
                    };
                    
                    if let Err(e) = executor.store_result(task_id, completion).await {
                        eprintln!("Failed to store result: {:?}", e);
                    }
                }
                Err(e) => {
                        let _ = executor.store_failure(task_id).await;
                        eprintln!("Failed to execute task with error: {:?}", e);
                }
            }
            progress_handler.abort();
        }
     });


    Ok(Json(TaskResponse {
        task_id: task_id.to_string(),
        state: TaskState::Running(
            initial_metrics
        ),
    }))
}