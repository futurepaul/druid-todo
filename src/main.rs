use druid::{AppLauncher, WindowDesc};

mod double_click;

mod data;
use data::*;

mod view;
use view::*;

pub fn main() {
    let main_window = WindowDesc::new(build_ui)
        .title("Todo")
        .window_size((400.0, 400.0));

    let initial_state = AppState::load_from_json();

    AppLauncher::with_window(main_window)
        .delegate(Delegate {})
        .launch(initial_state)
        .expect("Failed to launch application");
}
