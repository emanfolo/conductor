/* generated using openapi-typescript-codegen -- do not edit */
/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */
export type PrimeCalculationProgress = {
    /**
     * The current number being checked
     */
    current_number: number;
    /**
     * Number of primes found so far
     */
    found_primes: number;
    /**
     * Percentage of completion
     */
    percentage_complete?: number;
    /**
     * Current memory usage in bytes
     */
    current_memory_usage?: number;
    /**
     * Time elapsed in milliseconds
     */
    elapsed_time_ms?: number;
};

