use crate::types::prime_calculation::PrimeCalculationMetrics;

#[derive(Debug, Clone)]
pub enum TaskState {
    Running,
    Completed(Vec<u64>, PrimeCalculationMetrics),
    Failed(String),
}

#[derive(Debug)]
pub struct TaskError {
    message: String,
}

#[derive(Debug)]
pub struct TaskResponse {
    pub task_id: String,
    pub state: TaskState,
}