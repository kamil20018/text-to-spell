mod app;
mod state_manager;
mod states;
mod ui;

use app::App;

fn main() {
    let mut app = App::new();
    app.run();
}
