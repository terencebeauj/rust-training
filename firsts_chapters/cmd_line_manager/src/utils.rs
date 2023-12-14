use std::io;

pub fn get_user_task() -> String {
    let mut user_task: String = String::new();
    io::stdin()
        .read_line(&mut user_task)
        .expect("You entered a wrong string");
    user_task
}
