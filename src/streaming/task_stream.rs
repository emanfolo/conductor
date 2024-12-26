use std::{convert::Infallible, sync::Arc, time::Duration};
use axum::{extract::State, response::{sse::Event, Sse}};
use tokio_stream::StreamExt;
use futures::stream::{self, Stream};

use crate::executor::task_executor::TaskExecutor;

pub struct TaskStateStream {
    task_executor: Arc<TaskExecutor>,
}

impl TaskStateStream {
    pub fn new(task_executor: Arc<TaskExecutor>) -> Self {
        Self { task_executor }
    }

    pub fn stream(self) -> impl Stream<Item = Result<Event, Infallible>> {
        stream::repeat_with(move || {
            let executor = self.task_executor.clone();
            
            async move {
                // Get all tasks from executor
                let tasks = executor.get_all_tasks().await;
                
                // Create event with all tasks and their state
                let event = Event::default()
                    .json_data(&tasks)
                    .unwrap_or_else(|_| Event::default().data("Error serializing tasks"));
                
                Ok(event)
            }
        })
        .then(|future| future)
        .throttle(Duration::from_millis(100))
    }
}

pub async fn stream_all_tasks(
    State(task_executor): State<Arc<TaskExecutor>>,
) -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    let stream = TaskStateStream::new(task_executor).stream();
    Sse::new(stream)
}