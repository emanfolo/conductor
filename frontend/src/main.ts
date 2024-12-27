import { Task } from "../types";
  
  // main.ts
  class Symphony {
    private tasks: Map<string, Task> = new Map();
    private gridElement: HTMLElement;
  
    constructor() {
      this.gridElement = document.getElementById('taskGrid')!;
      this.initializeSSE();
    }
  
    private initializeSSE() {
      const eventSource = new EventSource('/api/tasks/stream');
      eventSource.onmessage = (event) => {
        const task = JSON.parse(event.data) as Task;
        this.updateTask(task);
      };
    }
  
    private updateTask(task: Task) {
      this.tasks.set(task.id, task);
      this.render();
    }
  
    private render() {
      this.gridElement.innerHTML = Array.from(this.tasks.values())
        .map(task => this.createTaskCard(task))
        .join('');
    }
  
    private createTaskCard(task: Task): string {
      return `
        <div class="bg-white rounded-lg shadow p-4 ${this.getStatusColor(task.state)}">
          <div class="flex justify-between items-center">
            <h3 class="text-lg font-semibold">Task ${task.id.slice(0, 8)}</h3>
            <span class="px-2 py-1 rounded text-sm ${this.getStatusBadgeColor(task.state)}">
              ${task.state}
            </span>
          </div>
          <div class="mt-4 space-y-2">
            <div class="flex justify-between">
              <span>Progress:</span>
              <span>${task.metrics.percentage_complete.toFixed(1)}%</span>
            </div>
            <div class="flex justify-between">
              <span>Memory:</span>
              <span>${(task.metrics.current_memory_bytes / 1024).toFixed(2)} KB</span>
            </div>
            <div class="flex justify-between">
              <span>Time:</span>
              <span>${(task.metrics.elapsed_time_ms / 1000).toFixed(2)}s</span>
            </div>
          </div>
        </div>
      `;
    }
  
    private getStatusColor(status: Task['state']): string {
      return {
        Running: 'border-l-4 border-blue-500',
        Completed: 'border-l-4 border-green-500',
        Failed: 'border-l-4 border-red-500',
      }[status];
    }
  
    private getStatusBadgeColor(status: Task['state']): string {
      return {
        Running: 'bg-blue-100 text-blue-800',
        Completed: 'bg-green-100 text-green-800',
        Failed: 'bg-red-100 text-red-800',
      }[status];
    }
  }
  
  // Initialize on page load
  new Symphony();