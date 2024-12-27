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