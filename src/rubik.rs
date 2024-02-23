use crate::control::{SmoothOrbitControl, SmoothOrbitControlSettings};
use crate::WindowLike;
use three_d::*;

mod geometry;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Color {
    Blue,
    Yellow,
    Red,
    White,
    Green,
    Orange,
}

static COLORS: [Color; 6] = [Color::Blue, Color::Yellow, Color::Red, Color::White, Color::Green, Color::Orange];

impl Color {
    pub fn iter() -> impl Iterator<Item = &'static Color> {
        COLORS.iter()
    }

    pub fn from_repr(repr: u8) -> Option<Self> {
        if repr < 6 {
            Some(COLORS[repr as usize])
        } else {
            None
        }
    }
}

pub struct Cube<M: Material> {
    pub cubes: Vec<Gm<Mesh, M>>,
    pub state: Vec<Color>,
    pub size: u8
}

impl<M: Material> Cube<M> {
    pub fn new(dim: u8) -> Self {
        let mut state = vec![Color::Blue; (6*dim*dim).into()];

        for i in 1..6 {
            for j in 0..dim*dim {
                state[(j + i*dim*dim) as usize] = COLORS[i as usize];
            }
        }

        Self {
            cubes: vec![],
            state,
            size: dim
        }
    }
}

pub fn run(window: &impl WindowLike) -> impl 'static + FnMut(FrameInput) -> FrameOutput {
    let context = window.gl();

    let mut camera = Camera::new_perspective(
        window.viewport(),
        vec3(4.5, 0.0, 4.5),
        vec3(0.0, 0.0, 0.0),
        vec3(0.0, 1.0, 0.0),
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

    const FACES: [Srgba; 6] = [
        Srgba::new(248, 214, 73, 255),
        Srgba::new(255, 255, 255, 255),
        Srgba::new(167, 41, 55, 255),
        Srgba::new(235, 99, 45, 255),
        Srgba::new(31, 68, 166, 255),
        Srgba::new(70, 152, 81, 255),
    ];
    let mut vec_colors = vec![Srgba::RED; 36];
    for i in 0..6 {
        let face_color = FACES[i];
        for j in 0..6 {
            vec_colors[i * 6 + j] = face_color;
        }
    }

    let mesh = CpuMesh {
        positions: CpuMesh::cube().positions,
        colors: Some(vec_colors),
        ..Default::default()
    };

    let model = Gm::new(Mesh::new(&context, &mesh), ColorMaterial::default());

    move |mut frame_input| {
        frame_input
            .screen()
            .clear(ClearState::color_and_depth(0.8, 0.8, 0.8, 0.8, 1.0))
            .render(&camera, &model, &[]);
        let dt: f32 = frame_input.elapsed_time as f32;
        camera.set_viewport(frame_input.viewport);
        control.handle_events(&mut camera, &mut frame_input.events, dt);
        FrameOutput::default()
    }
}
