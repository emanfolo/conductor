export interface Task {
    id: string;
    state: 'Running' | 'Completed' | 'Failed';
    metrics: {
      numbers_checked: number;
      current_memory_bytes: number;
      elapsed_time_ms: number;
      percentage_complete: number;
    };
  }

  export type PrimeCalculationMetrics = {
    current_memory_usage: number;
    current_number: number;
    elapsed_time_ms: number;
    found_primes: number;
    percentage_complete: number;

  }
  
 export type TaskStatus = {
    Running?: {
      metrics: {
        PrimeCalculationMetrics: PrimeCalculationMetrics
      };
    }
  }