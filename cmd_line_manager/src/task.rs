use crate::task_status::TaskStatus;

#[derive(Debug)]
pub struct Task {
    pub title: String,
    pub status: TaskStatus,
}
