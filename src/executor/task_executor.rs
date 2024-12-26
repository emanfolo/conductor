use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock; 
use uuid::Uuid;


use crate::executor::{TaskState, TaskError, CompletedMetrics, ProgressMetrics};

pub struct TaskExecutor {
    tasks: Arc<RwLock<HashMap<Uuid, TaskState>>>,
}

impl TaskExecutor {
    pub fn new() -> Self {
        Self {
            tasks: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn register_task(&self, task_id: Uuid, metrics: ProgressMetrics) -> Result<(), TaskError> {
        let mut tasks = self.tasks.write().await;
        tasks.insert(task_id, TaskState::Running(metrics));
        Ok(())
    }

    pub async fn store_result(
        &self, 
        task_id: Uuid, 
        metrics: CompletedMetrics
    ) -> Result<(), TaskError> {
        let mut tasks = self.tasks.write().await;
        tasks.insert(task_id, TaskState::Completed(metrics));
        Ok(())
    }

    pub async fn store_failure(
        &self, 
        task_id: Uuid,
    ) -> Result<(), TaskError> {
        let mut tasks = self.tasks.write().await;
        tasks.insert(task_id, TaskState::Failed("Task failed with error message".to_string()));
        Ok(())
    }

    pub async fn get_all_tasks(&self) -> HashMap<Uuid, TaskState> {
        let tasks = self.tasks.read().await;
        tasks.clone()
    }
}