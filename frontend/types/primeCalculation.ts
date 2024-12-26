  interface PrimeCalculationRequest {
    upperBound: number;
    batchSize?: number;
  }
  
  interface PrimeProgress {
    currentNumber: number;
    foundPrimes: number;
    percentageComplete: number;
    currentMemoryUsage: number;
    elapsedTimeMs: number;
  }
  
  interface PrimeCalculationMetrics {
    totalTimeMs: number;
    maxMemoryBytes: number;
    numbersChecked: number;
    averageCheckTimeNs: number;
  }