mod state;

use state::AppState;

fn main() {
    let app: AppState = AppState::Loading(true);
    app.print_state();
}
