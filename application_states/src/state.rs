pub enum AppState {
    Loading(bool),
    Running(bool),
    Error(bool),
    Exit(bool),
}

impl AppState {
    pub fn print_state(&self) {
        match self {
            AppState::Loading(_) => println!("Application is loading"),
            AppState::Running(_) => println!("Application is running"),
            AppState::Error(_) => println!("An error was raised in the application"),
            AppState::Exit(_) => println!("Going to exit the application"),
        }
    }
}