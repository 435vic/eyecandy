use crate::control::{SmoothOrbitControl, SmoothOrbitControlSettings};
use crate::WindowLike;
use three_d::*;

use super::{CubeAnimationOptions, Move};

#[derive(Clone, Default)]
pub struct RubikMaterial {
    pub render_states: RenderStates,
}

impl Material for RubikMaterial {
    fn id(&self) -> u16 {
        0u16
    }

    fn fragment_shader_source(&self, _lights: &[&dyn Light]) -> String {
        include_str!("rubik.frag").to_string()
    }

    fn fragment_attributes(&self) -> FragmentAttributes {
        FragmentAttributes {
            color: true,
            normal: true,
            uv: true,
            ..FragmentAttributes::NONE
        }
    }

    fn use_uniforms(&self, _: &Program, _: &Camera, _: &[&dyn Light]) {}

    fn render_states(&self) -> RenderStates {
        self.render_states
    }

    fn material_type(&self) -> MaterialType {
        MaterialType::Opaque
    }
}

pub fn run(window: &impl WindowLike) -> impl 'static + FnMut(FrameInput) -> FrameOutput {
    let context = window.gl();

    let mut camera = Camera::new_perspective(
        window.viewport(),
        Vec3::new(4.5, 0.0, 4.5),
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        degrees(45.0),
        0.1,
        50.0,
    );

    let mut control = SmoothOrbitControl::new(
        *camera.target(),
        &camera,
        SmoothOrbitControlSettings {
            min_zoom: 4.0,
            ..Default::default()
        },
    );

    // x axis red
    // y axis green
    // z axis blue
    let axes = Axes::new(&context, 0.08, 5.0);
    let mut cooler_rubik = super::Cube::solved(CubeAnimationOptions::default(), &context);
    cooler_rubik.queue([Move::L, Move::F, Move::L2, Move::UP, Move::B, Move::RP, Move::LP].into_iter());

    move |mut frame_input| {
        // cooler_rubik.animate(frame_input.accumulated_time as f32);

        frame_input
            .screen()
            .clear(ClearState::color_and_depth(0.2, 0.2, 0.2, 0.8, 1.0))
            .render(&camera, cooler_rubik.into_iter().chain(&axes), &[]);
            // .render(&camera, rubik.into_iter().chain(&axes), &[]);
        let dt: f32 = frame_input.elapsed_time as f32;
        camera.set_viewport(frame_input.viewport);
        control.handle_events(&mut camera, &mut frame_input.events, dt);
        FrameOutput::default()
    }
}
