use serde::{Deserialize, Serialize};

use crate::types::{prime_calculation::{PrimeCalculationCompletedMetrics, PrimeCalculationProgressMetrics}, visualisation::VisualisationFrame};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ProgressMetrics {
    PrimeCalculationMetrics(PrimeCalculationProgressMetrics)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CompletedMetrics {
    PrimeCalculationMetrics(PrimeCalculationCompletedMetrics)
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskProgress {
    pub metrics: ProgressMetrics,
    pub visualisation: Option<Vec<VisualisationFrame>>,
    pub timestamp: u64,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskCompletion {
    pub metrics: CompletedMetrics,
    pub visualisation: Vec<VisualisationFrame>,
    pub timestamp: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TaskState {
    Running(TaskProgress),
    Completed(TaskCompletion),
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