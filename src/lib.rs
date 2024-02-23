pub mod animation;
pub mod control;
pub mod rubik;
pub mod wasm;

use three_d::{Context, Viewport};

pub trait WindowLike {
    fn gl(&self) -> Context;
    fn viewport(&self) -> Viewport;
    fn size(&self) -> (u32, u32);
}

#[cfg(target_family = "wasm")]
impl WindowLike for wasm::window::Window {
    fn gl(&self) -> Context {
        self.gl()
    }
    fn viewport(&self) -> Viewport {
        self.viewport()
    }
    fn size(&self) -> (u32, u32) {
        self.size()
    }
}

impl WindowLike for three_d::window::Window {
    fn gl(&self) -> Context {
        self.gl()
    }
    fn viewport(&self) -> Viewport {
        self.viewport()
    }
    fn size(&self) -> (u32, u32) {
        self.size()
    }
}
