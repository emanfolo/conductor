import {PrimeCalculationMetrics, TaskStatus} from "../types"

interface Column {
  x: number;
  y: number;
  speed: number;
  chars: string[];
  prime: boolean;
  intensity: number;
}

class MatrixVisualization {
  private canvas: HTMLCanvasElement;
  private ctx: CanvasRenderingContext2D;
  private columns: Column[] = [];
  private fontSize: number = 14;
  private isRunning: boolean = false;
  private animationFrame: number | null = null;
  private taskMetrics: Map<string, PrimeCalculationMetrics> = new Map();

  constructor(canvasId: string, metricsId: string) {
    const canvas = document.getElementById(canvasId) as HTMLCanvasElement;
    const metricsEl = document.getElementById(metricsId);
    
    if (!canvas || !metricsEl) throw new Error('Canvas or metrics container not found');
    
    this.canvas = canvas;
    this.ctx = canvas.getContext('2d')!;
    this.initializeCanvas();
    this.initializeMetrics(metricsEl);
    
    // Debounced resize handler
    let resizeTimeout;
    window.addEventListener('resize', () => {
      clearTimeout(resizeTimeout);
      resizeTimeout = setTimeout(() => this.initializeCanvas(), 250);
    });
  }

  private initializeCanvas(): void {
    this.canvas.width = this.canvas.offsetWidth;
    this.canvas.height = this.canvas.offsetHeight;
    this.resetColumns();
  }

  private resetColumns(): void {
    const columnCount = Math.floor(this.canvas.width / this.fontSize);
    this.columns = Array(columnCount).fill(0).map(() => ({
      x: Math.random() * this.canvas.width,
      y: Math.random() * this.canvas.height,
      speed: 1 + Math.random() * 2,
      chars: this.generateRandomChars(),
      prime: false,
      intensity: this.isRunning ? 1 : 0.3
    }));
  }

  private generateRandomChars(length = 20): string[] {
    return Array(Math.floor(length + Math.random() * 10))
      .fill(0)
      .map(() => Math.floor(Math.random() * 10).toString());
  }
  private initializeMetrics(container: HTMLElement): void {
    container.innerHTML = `<div class="grid grid-cols-1 md:grid-cols-2 gap-4" id="task-metrics-container"></div>`;
  }

  private createTaskBox(taskId: string): HTMLElement {
    const taskBox = document.createElement('div');
    taskBox.id = `task-${taskId}`;
    taskBox.className = 'bg-black border border-green-400/20 p-4 rounded-lg space-y-2';
    
    taskBox.innerHTML = `
      <div class="flex justify-between items-center border-b border-green-400/20 pb-2">
        <h3 class="text-lg font-semibold text-green-400">${taskId}</h3>
        <span class="text-sm text-green-400/60" id="${taskId}-status">Running</span>
      </div>
      <div class="grid grid-cols-2 gap-2 text-sm">
        <div>
          <p class="text-green-400/60">Found Primes</p>
          <p class="text-lg font-bold text-green-400" id="${taskId}-found-primes">-</p>
        </div>
        <div>
          <p class="text-green-400/60">Progress</p>
          <p class="text-lg font-bold text-green-400" id="${taskId}-progress">-</p>
        </div>
        <div>
          <p class="text-green-400/60">Current Number</p>
          <p class="text-lg font-bold text-green-400" id="${taskId}-current-number">-</p>
        </div>
        <div>
          <p class="text-green-400/60">Elapsed Time</p>
          <p class="text-lg font-bold text-green-400" id="${taskId}-elapsed-time">-</p>
        </div>
        <div class="col-span-2">
          <p class="text-green-400/60">Memory Usage</p>
          <p class="text-lg font-bold text-green-400" id="${taskId}-memory-usage">-</p>
        </div>
      </div>
    `;

    return taskBox;
  }

  private startAnimation(): void {
    if (this.animationFrame) return;

    const draw = () => {
      // Fade effect
      this.ctx.fillStyle = 'rgba(0, 0, 0, 0.05)';
      this.ctx.fillRect(0, 0, this.canvas.width, this.canvas.height);

      this.ctx.font = `${this.fontSize}px monospace`;
      
      this.columns.forEach(column => {
        // Only animate if task is running or column still has intensity
        if (this.isRunning || column.intensity > 0.1) {
          this.drawColumn(column);
          this.updateColumn(column);
        }
      });
      
      this.animationFrame = requestAnimationFrame(draw);
    };

    draw();
  }

  private drawColumn(column: Column): void {
    column.chars.forEach((char, j) => {
      const y = (column.y + j * this.fontSize) % this.canvas.height;
      
      this.ctx.fillStyle = column.prime 
        ? `rgba(0, 255, 70, ${Math.max(0.2, column.intensity)})`
        : `rgba(0, 255, 70, ${Math.max(0.1, column.intensity) * 0.3})`;
      
      this.ctx.fillText(char, column.x, y);
    });
  }

  private updateColumn(column: Column): void {
    column.y += column.speed;
    // Fade out slower when not running
    column.intensity *= this.isRunning ? 0.99 : 0.995;
  }

  public startTask(): void {
    this.isRunning = true;
    this.columns.forEach(column => column.intensity = 1);
    this.startAnimation();
  }

  public stopTask(): void {
    this.isRunning = false;
    // Let the animation fade out naturally
  }

  public removeTask(taskId: string): void {
    const taskBox = document.getElementById(`task-${taskId}`);
    if (taskBox) {
      taskBox.classList.add('opacity-50');
      const statusElement = document.getElementById(`${taskId}-status`);
      const progressElement = document.getElementById(`${taskId}-progress`);
      if (statusElement) {
        statusElement.textContent = 'Completed';
      }
      if (progressElement) {
        progressElement.textContent = '100%'
      }
    }
    this.taskMetrics.delete(taskId);
  }



  public updateMetrics(taskId: string, metrics: PrimeCalculationMetrics): void {
    if (!this.isRunning) this.startTask();
    
    const container = document.getElementById('task-metrics-container');
    if (!container) return;

    // Create task box if it doesn't exist
    if (!document.getElementById(`task-${taskId}`)) {
      container.appendChild(this.createTaskBox(taskId));
    }

    const formatters = {
      number: (num: number) => new Intl.NumberFormat().format(num),
      percent: (num: number) => `${num.toFixed(2)}%`,
      time: (ms: number) => `${(ms / 1000).toFixed(1)}s`,
      memory: (bytes: number) => `${(bytes / 1024).toFixed(1)}KB`
    };

    // Update metrics for this task
    const updates = {
      'found-primes': formatters.number(metrics.found_primes),
      'progress': formatters.percent(metrics.percentage_complete),
      'current-number': formatters.number(metrics.current_number),
      'elapsed-time': formatters.time(metrics.elapsed_time_ms),
      'memory-usage': formatters.memory(metrics.current_memory_usage)
    };

    Object.entries(updates).forEach(([id, value]) => {
      const element = document.getElementById(`${taskId}-${id}`);
      if (element) element.textContent = value;
    });

    this.taskMetrics.set(taskId, metrics);
  }

  public highlightPrime(number: number): void {
    const highlightCount = 2 + Math.floor(Math.random() * 2);
    const primeStr = number.toString();
    
    for (let i = 0; i < highlightCount; i++) {
      const column = this.columns[Math.floor(Math.random() * this.columns.length)];
      column.prime = true;
      column.intensity = 1;
      column.chars = [...primeStr, ...this.generateRandomChars()];
    }
  }

  public destroy(): void {
    if (this.animationFrame) {
      cancelAnimationFrame(this.animationFrame);
      this.animationFrame = null;
    }
  }
}

class Symphony {
  private visualization: MatrixVisualization;
  private eventSource: EventSource | null = null;

  constructor() {
    this.visualization = new MatrixVisualization('visualizationCanvas', 'metricsContainer');
    this.initializeSSE();
  }

  private initializeSSE(): void {
    this.eventSource = new EventSource('http://localhost:5001/api/stream');
    this.eventSource.onmessage = (event: MessageEvent) => {
      const taskData: TaskStatus[] = JSON.parse(event.data);
      console.log(taskData)
      this.visualization.startTask();

      
      taskData.forEach((data, index) => {
        const taskId = `Task-${index}`;
        if (data.Running) {
          this.visualization.updateMetrics(taskId, data.Running.metrics.PrimeCalculationMetrics);
          // this.visualization.highlightPrime(data.Running.metrics.PrimeCalculationMetrics.current_number)

        } else {
          this.visualization.removeTask(taskId);
          this.visualization.stopTask();        
        }
      });
    };
  }

  public destroy(): void {
    this.visualization.destroy();
    if (this.eventSource) {
      this.eventSource.close();
      this.eventSource = null;
    }
  }
}

new Symphony()