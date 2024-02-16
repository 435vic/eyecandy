#![cfg(target_family = "wasm")]

use std::sync::OnceLock;
use three_d::SurfaceSettings;
use winit::event_loop::EventLoop;
use wasm_bindgen::prelude::*;
use web_sys::HtmlCanvasElement;
use crate::demo::run_demo;
use log::info;

pub mod window;

static EVENT_LOOP_CREATED: OnceLock<()> = OnceLock::new();
static mut _SHOULD_EXIT: bool = false;

#[wasm_bindgen(start)]
pub fn wasm_start() -> Result<(), JsValue> {
    console_log::init_with_level(log::Level::Debug).unwrap();
    info!("Log hook registered!");

    console_error_panic_hook::set_once();
    info!("Panic hook registered!");

    Ok(())
}

#[wasm_bindgen]
pub fn bind(canvas: HtmlCanvasElement) -> Result<(), JsValue> {
    if let Err(_) = EVENT_LOOP_CREATED.set(()) {
        return Err(JsValue::from_str("Event loop already created."));
    }
    let event_loop = EventLoop::new();
    let window = window::Window::new(
        canvas,
        &event_loop,
        SurfaceSettings::default()
    );
    let closure = run_demo(&window);
    window.start(event_loop, closure);
    Ok(())
}
