use three_d::{Window, WindowSettings};
use eyecandy::demo::run_demo;

use env_logger::Env;

pub fn main() {
    env_logger::Builder::from_env(
        Env::default()
            .default_filter_or("trace")
    ).format_timestamp(None).init();
    let window = Window::new(WindowSettings {
        title: "Demo".to_string(),
        max_size: Some((800, 600)),
        ..Default::default()
    }).unwrap();
    let closure = run_demo(&window);
    window.render_loop(closure);
}