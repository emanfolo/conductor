use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct PrimeCalculationRequest {
    pub upper_bound: u64,
    pub batch_size: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PrimeCalculationMetrics {
    pub total_time_ms: u64,
    pub max_memory_bytes: u64,
    pub numbers_checked: u64,
    pub average_check_time_ns: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PrimeProgress {
    pub current_number: u64,
    pub found_primes: u32,
    pub percentage_complete: f32,
    pub current_memory_usage: u64,
    pub elapsed_time_ms: u64,
}