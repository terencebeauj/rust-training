mod task_manager;
mod task;
mod task_status;
mod utils;

use crate::task_manager::TaskManager;
use std::io;

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
                let user_task: String = utils::get_user_task();
                task_manager.add_task(user_task.trim_end());
            }
            "2" => {
                println!("You want to list all your tasks\n");
                task_manager.list_tasks();
            }
            "3" => {
                println!("You want to delete a task. What task do you want to delete ?\n");
                let user_task: String = utils::get_user_task();
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
