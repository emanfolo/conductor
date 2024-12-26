use serde::{Deserialize, Serialize};

use crate::types::prime_calculation::{PrimeCalculationCompletedMetrics, PrimeCalculationProgressMetrics};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ProgressMetrics {
    PrimeCalculationMetrics(PrimeCalculationProgressMetrics)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CompletedMetrics {
    PrimeCalculationMetrics(PrimeCalculationCompletedMetrics)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TaskState {
    Running(ProgressMetrics),
    Completed(CompletedMetrics),
    Failed(String),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TaskError {
    message: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TaskResponse {
    pub task_id: String,
    pub state: TaskState,
}