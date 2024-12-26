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