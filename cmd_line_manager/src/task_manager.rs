use crate::task::Task;
use crate::task_status::TaskStatus;

#[derive(Debug)]
pub struct TaskManager {
    pub tasks: Vec<Task>,
}

impl TaskManager {
    pub fn add_task(&mut self, title: &str) {
        let task: Task = Task {
            title: String::from(title),
            status: TaskStatus::Pending,
        };

        self.tasks.push(task);
        println!("Successfully added {}", title);
    }

    pub fn remove_task(&mut self, title: &str) {
        let mut index_to_delete: Option<usize> = None;
        for (i, task) in self.tasks.iter().enumerate() {
            if task.title == title {
                index_to_delete = Some(i);
            }
        }

        match index_to_delete {
            Some(i) => {
                self.tasks.remove(i);
                println!("Successfully removed {}", title);
            }
            None => {
                println!("{} does not exists in the current tasks", title);
            }
        };
    }

    pub fn update_tasks(&mut self) {
        for task in &mut self.tasks {
            match task.status {
                TaskStatus::Pending => task.status = TaskStatus::InProgress,
                TaskStatus::InProgress => task.status = TaskStatus::Completed,
                _ => (),
            };
        }
    }

    pub fn list_tasks(&self) {
        dbg!(&self.tasks);
    }

    pub fn clean(&mut self) {
        self.tasks
            .retain(|task| task.status != TaskStatus::Completed);
    }
}
