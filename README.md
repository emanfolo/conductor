# Rust Task Executor

A concurrent task processing system built in Rust with real-time monitoring and a web interface.

## Project Goals
- Learn Rust's memory management and ownership model
- Understand concurrent programming patterns
- Build a system that handles resource-intensive tasks
- Implement real-time monitoring and visualization

## Core Features
1. Task Processing
   - Concurrent task execution
   - Resource monitoring (CPU, memory)
   - Progress tracking
   - Task cancelation

2. Web Interface
   - Single task creation
   - Batch task generation ("Chaos Mode")
   - Real-time progress updates
   - System metrics visualization

3. Task Types
   - CPU-bound: Prime number calculation, Matrix multiplication
   - Memory-bound: Large array sorting, Memory allocation stress test
   - I/O-bound: File operations simulation

## Development Schedule

### Week 1: Core Implementation
#### Days 1-2: Foundation
- [ ] Basic task struct and enum definitions
- [ ] Single-threaded task executor
- [ ] Basic CLI for testing task execution

#### Days 3-4: Concurrency
- [ ] Concurrent task executor using tokio
- [ ] Task state management
- [ ] Basic resource monitoring

#### Days 5-7: API Layer
- [ ] REST API with axum
- [ ] Basic web interface setup with Astro
- [ ] Task creation and status endpoints

### Week 2: Features & Polish
#### Days 1-2: Task Implementation
- [ ] Implement various task types
- [ ] Add progress tracking
- [ ] Implement task cancellation

#### Days 3-4: Monitoring & Testing
- [ ] System metrics collection
- [ ] K6 load testing setup
- [ ] "Chaos Mode" for batch task generation
- [ ] Performance metrics dashboard

#### Days 5-7: UI & Integration
- [ ] Real-time updates using WebSocket
- [ ] Task performance visualization
- [ ] System health monitoring
- [ ] Final testing and documentation

## Technical Stack
- Backend: Rust (axum, tokio)
- Frontend: Astro + TypeScript
- Testing: K6 for load testing
- Monitoring: Custom metrics collection

## Stretch Goals
- [ ] Task priorities and queuing
- [ ] Resource limits per task
- [ ] Task dependencies
- [ ] Persistence layer
- [ ] Docker containerization