use std::time::{Instant, Duration};
use tokio::sync::mpsc;
use crate::types::prime_calculation::{PrimeCalculationCompletedMetrics, PrimeCalculationProgressMetrics};

pub struct PrimeCalculator {
    upper_bound: u64,
    batch_size: u32,
    progress_sender: mpsc::Sender<PrimeCalculationProgressMetrics>,
}

impl PrimeCalculator {
    pub fn new(
        upper_bound: u64, 
        batch_size: u32,
        progress_sender: mpsc::Sender<PrimeCalculationProgressMetrics>
    ) -> Self {
        Self {
            upper_bound,
            batch_size,
            progress_sender,
        }
    }

    fn is_prime(n: u64) -> bool {
        if n <= 1 {
            return false;
        }
        if n <= 3 {
            return true;
        }
        if n % 2 == 0 || n % 3 == 0 {
            return false;
        }

        let sqrt_n = (n as f64).sqrt() as u64;
        let mut i = 5;
        while i <= sqrt_n {
            if n % i == 0 || n % (i + 2) == 0 {
                return false;
            }
            i += 6;
        }
        true
    }

    pub async fn calculate(&self) -> Result<PrimeCalculationCompletedMetrics, String> {
        println!("Starting calculation for upper_bound: {}", self.upper_bound);
        let start_time = Instant::now();
        let mut primes = Vec::new();
        let mut numbers_checked = 0u64;
        let mut last_progress_update = Instant::now();

        // Optimization: Start with known small primes
        primes.extend_from_slice(&[2, 3]);
        
        let mut current = 5u64;
        while current <= self.upper_bound {
            numbers_checked += 1;

            if Self::is_prime(current) {
                primes.push(current);
            }

            // Send progress updates based on batch_size
            if numbers_checked % self.batch_size as u64 == 0 
                || last_progress_update.elapsed() >= Duration::from_secs(1) 
            {
                let progress = PrimeCalculationProgressMetrics {
                    current_number: current,
                    found_primes: primes.len() as u32,
                    percentage_complete: (current as f32 / self.upper_bound as f32) * 100.0,
                    current_memory_usage: primes.capacity() as u64 * std::mem::size_of::<u64>() as u64,
                    elapsed_time_ms: start_time.elapsed().as_millis() as u64,
                };

                let _ = self.progress_sender.send(progress).await;

                last_progress_update = Instant::now();
            }

            current += 2; // Skip even numbers
        }

        let total_time = start_time.elapsed();

        let max_memory_bytes = primes.capacity() as u64 * std::mem::size_of::<u64>() as u64;
        
        let metrics = PrimeCalculationCompletedMetrics {
            found_primes: primes.len() as u32,
            total_time_ms: total_time.as_millis() as u64,
            max_memory_bytes,
            numbers_checked,
            average_check_time_ns: total_time.as_nanos() as f64 / numbers_checked as f64,
        };
        println!("Calculation complete");
        Ok(metrics)
    }
}
