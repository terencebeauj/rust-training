enum TaskStatus {
    Pending,
    InProgress,
    Completed,
}

struct Task {
    title: String,
    status: TaskStatus,
}

struct TaskManager {
    tasks: Vec<Task>,
}

impl TaskManager {
    fn add_task(&mut self, title: &str) {

    }

    fn remove_task(&mut self, title: &str) {

    }

    fn update_task(&self, title: &str) {

    }

    fn list_task(&self) -> Vec<&Task> {
        let mut results: Vec<&Task> = vec![];

        results
    }
}

fn main() {
    println!("Hello, world!");
}
