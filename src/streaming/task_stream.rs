use std::{convert::Infallible, sync::Arc};

use axum::{extract::{Path, State}, response::{sse::Event, Sse}};
use uuid::Uuid;
use futures::stream::{self, Stream};

use crate::executor::task_executor::TaskExecutor;

pub struct TaskStateStream {
    task_executor: Arc<TaskExecutor>,
    task_id: Uuid,
}

impl TaskStateStream {
    pub fn new(task_executor: Arc<TaskExecutor>, task_id: Uuid) -> Self {
        Self { task_executor, task_id }
    }

    pub fn stream(self) -> impl Stream<Item = Result<Event, Infallible>> {
        stream::unfold(self, |state| async move {
            let event = Event::default().data("Hi");
            Some((Ok(event), state))
        })
    }
}

pub async fn stream_task_state(
    Path(task_id): Path<Uuid>,
    State(task_executor): State<Arc<TaskExecutor>>, 
) -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    let stream = TaskStateStream::new(task_executor, task_id).stream();
    Sse::new(stream)
}