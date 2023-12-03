use std::io;

#[derive(Debug, PartialEq)]
enum TaskStatus {
    Pending,
    InProgress,
    Completed,
}

#[derive(Debug)]
struct Task {
    title: String,
    status: TaskStatus,
}

#[derive(Debug)]
struct TaskManager {
    tasks: Vec<Task>,
}

impl TaskManager {
    fn add_task(&mut self, title: &str) {
        let task: Task = Task {
            title: String::from(title),
            status: TaskStatus::Pending,
        };

        self.tasks.push(task);
        println!("Successfully added {}", title);
    }

    fn remove_task(&mut self, title: &str) {
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

    fn update_tasks(&mut self) {
        for task in &mut self.tasks {
            match task.status {
                TaskStatus::Pending => task.status = TaskStatus::InProgress,
                TaskStatus::InProgress => task.status = TaskStatus::Completed,
                _ => (),
            };
        }
    }

    fn list_tasks(&self) {
        dbg!(&self.tasks);
    }

    fn clean(&mut self) {
        self.tasks
            .retain(|task| task.status != TaskStatus::Completed);
    }
}

fn get_user_task() -> String {
    let mut user_task: String = String::new();
    io::stdin()
        .read_line(&mut user_task)
        .expect("You entered a wrong string");
    user_task
}

fn main() {
    let mut task_manager: TaskManager = TaskManager { tasks: vec![] };

    loop {
        let mut user_task: String = String::new();

        println!("Welcome to your Task Manager Software\n");
        println!("You can:\n1 - Create a task\n2 - List your different tasks\n3 - Delete a task\n4 - Clean\n5 - Exit");
        println!("Please enter your choice: ");

        io::stdin()
            .read_line(&mut user_task)
            .expect("Your string is invalid.");

        let user_task: &str = user_task.trim_end();

        match user_task {
            "1" => {
                println!("You want to create a task. What is your task ?\n");
                let user_task: String = get_user_task();
                task_manager.add_task(user_task.trim_end());
            }
            "2" => {
                println!("You want to list all your tasks\n");
                task_manager.list_tasks();
            }
            "3" => {
                println!("You want to delete a task. What task do you want to delete ?\n");
                let user_task: String = get_user_task();
                task_manager.remove_task(user_task.trim_end());
            }
            "4" => {
                println!("Cleaning resources...");
                task_manager.clean();
            }
            "5" => {
                println!("Exiting software...");
                break;
            }
            _ => {
                println!("You entered a wrong number, you can enter only 1, 2, 3, 4 or 5 !");
            }
        };

        task_manager.update_tasks();
        println!("\n\n");
    }
}
