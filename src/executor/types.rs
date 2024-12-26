use serde::{Deserialize, Serialize};

use crate::types::prime_calculation::PrimeCalculationMetrics;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TaskState {
    Running,
    Completed(Vec<u64>, PrimeCalculationMetrics),
    Failed(String),
}

#[derive(Debug,)]
pub struct TaskError {
    message: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TaskResponse {
    pub task_id: String,
    pub state: TaskState,
}