/* generated using openapi-typescript-codegen -- do not edit */
/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */
export type PrimeCalculationResult = {
    /**
     * List of all prime numbers found
     */
    primes: Array<number>;
    execution_stats: {
        /**
         * Total execution time in milliseconds
         */
        total_time_ms: number;
        /**
         * Peak memory usage in bytes
         */
        max_memory_bytes: number;
        /**
         * Total numbers checked
         */
        numbers_checked: number;
        /**
         * Average time per number in nanoseconds
         */
        average_check_time_ns?: number;
    };
};

