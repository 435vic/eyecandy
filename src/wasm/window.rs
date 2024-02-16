use three_d::{Context, FrameInput, FrameInputGenerator, FrameOutput, SurfaceSettings, Viewport, WindowedContext};
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;
use web_sys::HtmlCanvasElement;
use winit::{
    dpi::LogicalSize, event, event_loop::EventLoop, window::WindowBuilder
};

///
/// The threedweb window. Uses winit as a backend.
///
pub struct Window {
    pub(super) window: winit::window::Window,
    pub(super) exit_closure: Closure<dyn FnMut(web_sys::Event)>,
    pub(super) gl: WindowedContext,
    pub(super) fig: FrameInputGenerator
}

impl Window {
    ///
    /// Create a new window.
    /// Requires an existing [HtmlCanvasElement] to bind to and an event loop.
    ///
    /// It is recommended to disable the `contextmenu` event on the canvas element with JavaScript
    /// before passing it to the handler. This way if you use the right click for anything in your
    /// code a random menu doesn't get in your way. You can do this by adding an event listener
    /// and calling `preventDefault` on the event object.
    ///
    /// Use [SurfaceSettings::default] as the surface settings if you're not sure what it's for.
    ///
    pub fn new(
        canvas: HtmlCanvasElement,
        event_loop: &EventLoop<()>,
        surface_settings: SurfaceSettings
    ) -> Self {
        let size = (canvas.width(), canvas.height());
        let builder = {
            use winit::platform::web::WindowBuilderExtWebSys;
            WindowBuilder::new()
                .with_canvas(Some(canvas))
                .with_inner_size::<LogicalSize<u32>>(LogicalSize::from(size))
                .with_prevent_default(true) // allows for custom controls in rendering
        };
        let window = builder.build(&event_loop).unwrap();
        // Create custom exit event to stop program on demand
        let closure: Closure<dyn FnMut(web_sys::Event)> = {
            use winit::platform::web::WindowExtWebSys;
            let closure =
                Closure::new(move |_| {
                    unsafe {
                        super::_SHOULD_EXIT = true;
                    }
                });
            // Register event on canvas element
            window.canvas()
                .add_event_listener_with_callback("__wasm_exit", closure.as_ref().unchecked_ref())
                .expect("Failed to add exit event to canvas");
            closure
        };
        let context =
            WindowedContext::from_winit_window(&window, surface_settings).unwrap();
        let frame_input_generator = FrameInputGenerator::from_winit_window(&window);
        Self {
            window,
            gl: context,
            exit_closure: closure,
            fig: frame_input_generator
        }
    }

    /// Returns the current viewport of the window in physical pixels (the size of the screen returned from [FrameInput::screen]).
    pub fn viewport(&self) -> Viewport {
        let (w, h): (u32, u32) = self.window.inner_size().into();
        Viewport::new_at_origo(w, h)
    }

    /// Get the graphics context of the window.
    pub fn gl(&self) -> Context {
        (*self.gl).clone()
    }

    /// Get the current logical size of the window.
    pub fn size(&self) -> (u32, u32) {
        self.window.inner_size()
            .to_logical::<f64>(self.window.scale_factor())
            .into()
    }

    pub fn start<F>(
        mut self, // Not a reference because event loop can only be started once
        event_loop: EventLoop<()>,
        mut render_callback: F
    ) where F: 'static + FnMut(FrameInput) -> FrameOutput {
        event_loop.run(move |event, _, control_flow| match &event {
            event::Event::LoopDestroyed => {
                use winit::platform::web::WindowExtWebSys;
                self.window.canvas()
                    .remove_event_listener_with_callback(
                        "__wasm_exit",
                        self.exit_closure.as_ref().unchecked_ref()
                    ).unwrap();
            }
            event::Event::MainEventsCleared => {
                self.window.request_redraw();
            }
            event::Event::RedrawRequested(_) => {
                let frame_input = self.fig.generate(&self.gl);
                let frame_output = render_callback(frame_input);
                let should_exit = unsafe { super::_SHOULD_EXIT };
                if frame_output.exit || should_exit {
                    control_flow.set_exit();
                } else if frame_output.wait_next_event {
                    control_flow.set_wait();
                } else {
                    control_flow.set_poll();
                    self.window.request_redraw();
                }
            }
            event::Event::WindowEvent { ref event, .. } => {
                self.fig.handle_winit_window_event(event);
            }
            _ => {}
        });
    }
}
