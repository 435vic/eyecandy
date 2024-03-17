use three_d_asset::Positions;
use crate::control::{SmoothOrbitControl, SmoothOrbitControlSettings};
use crate::WindowLike;
use three_d::*;

use super::Move;

/// Same as CpuMesh::cube() but the order of faces is changed to match my internal order
/// makes it easier to compute colors for the faces and also cube turns.
pub fn cube_mesh() -> CpuMesh {
    // The CpuMesh::cube() function has 6*6 = 36 vertices.
    // 3 vertices per triangle, 2 triangles per face
    // Up, down, back, front, right and left
    let positions = vec![
        // Left
        Vec3::new(-1.0, 1.0, -1.0),
        Vec3::new(-1.0, -1.0, -1.0),
        Vec3::new(-1.0, 1.0, 1.0),
        Vec3::new(-1.0, -1.0, 1.0),
        Vec3::new(-1.0, 1.0, 1.0),
        Vec3::new(-1.0, -1.0, -1.0),
        // Up
        Vec3::new(1.0, 1.0, -1.0),
        Vec3::new(-1.0, 1.0, -1.0),
        Vec3::new(1.0, 1.0, 1.0),
        Vec3::new(-1.0, 1.0, 1.0),
        Vec3::new(1.0, 1.0, 1.0),
        Vec3::new(-1.0, 1.0, -1.0),
        // Front
        Vec3::new(-1.0, -1.0, 1.0),
        Vec3::new(1.0, -1.0, 1.0),
        Vec3::new(1.0, 1.0, 1.0),
        Vec3::new(1.0, 1.0, 1.0),
        Vec3::new(-1.0, 1.0, 1.0),
        Vec3::new(-1.0, -1.0, 1.0),
        // Down
        Vec3::new(-1.0, -1.0, -1.0),
        Vec3::new(1.0, -1.0, -1.0),
        Vec3::new(1.0, -1.0, 1.0),
        Vec3::new(1.0, -1.0, 1.0),
        Vec3::new(-1.0, -1.0, 1.0),
        Vec3::new(-1.0, -1.0, -1.0),
        // Right
        Vec3::new(1.0, -1.0, -1.0),
        Vec3::new(1.0, 1.0, -1.0),
        Vec3::new(1.0, 1.0, 1.0),
        Vec3::new(1.0, 1.0, 1.0),
        Vec3::new(1.0, -1.0, 1.0),
        Vec3::new(1.0, -1.0, -1.0),
        // Back
        Vec3::new(1.0, -1.0, -1.0),
        Vec3::new(-1.0, -1.0, -1.0),
        Vec3::new(1.0, 1.0, -1.0),
        Vec3::new(-1.0, 1.0, -1.0),
        Vec3::new(1.0, 1.0, -1.0),
        Vec3::new(-1.0, -1.0, -1.0),
    ];

    let uvs = vec![
        // Left
        Vec2::new(0.25, 1.0 / 3.0),
        Vec2::new(0.25, 2.0 / 3.0),
        Vec2::new(0.5, 1.0 / 3.0),
        Vec2::new(0.5, 2.0 / 3.0),
        Vec2::new(0.5, 1.0 / 3.0),
        Vec2::new(0.25, 2.0 / 3.0),
        // Up
        Vec2::new(0.25, 0.0),
        Vec2::new(0.25, 1.0 / 3.0),
        Vec2::new(0.5, 0.0),
        Vec2::new(0.5, 1.0 / 3.0),
        Vec2::new(0.5, 0.0),
        Vec2::new(0.25, 1.0 / 3.0),
        // Front
        Vec2::new(0.5, 2.0 / 3.0),
        Vec2::new(0.75, 2.0 / 3.0),
        Vec2::new(0.75, 1.0 / 3.0),
        Vec2::new(0.75, 1.0 / 3.0),
        Vec2::new(0.5, 1.0 / 3.0),
        Vec2::new(0.5, 2.0 / 3.0),
        // Down
        Vec2::new(0.25, 2.0 / 3.0),
        Vec2::new(0.25, 1.0),
        Vec2::new(0.5, 1.0),
        Vec2::new(0.5, 1.0),
        Vec2::new(0.5, 2.0 / 3.0),
        Vec2::new(0.25, 2.0 / 3.0),
        // Right
        Vec2::new(1.0, 2.0 / 3.0),
        Vec2::new(1.0, 1.0 / 3.0),
        Vec2::new(0.75, 1.0 / 3.0),
        Vec2::new(0.75, 1.0 / 3.0),
        Vec2::new(0.75, 2.0 / 3.0),
        Vec2::new(1.0, 2.0 / 3.0),
        // Back
        Vec2::new(0.0, 2.0 / 3.0),
        Vec2::new(0.25, 2.0 / 3.0),
        Vec2::new(0.0, 1.0 / 3.0),
        Vec2::new(0.25, 1.0 / 3.0),
        Vec2::new(0.0, 1.0 / 3.0),
        Vec2::new(0.25, 2.0 / 3.0),
    ];

    let mut mesh = CpuMesh {
        positions: Positions::F32(positions),
        uvs: Some(uvs),
        ..Default::default()
    };

    mesh.compute_normals();
    mesh.compute_tangents();
    mesh
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
    let mut cooler_rubik = super::Cube::solved(&context);
    cooler_rubik.queue([Move::L, Move::F, Move::L, Move::UP].into_iter());
    // let _changed = cooler_rubik.face(4).map(|piece| {
    //     let result = piece.rotate(super::ROT_XY_CW).unwrap();
    //     println!("{:?}", result)
    //     result
    // }).collect::<Vec<_>>();
    

    move |mut frame_input| {
        // cooler_rubik._dbg_rotate_face_model(0, radians(frame_input.accumulated_time as f32 * 0.001)).unwrap();
        cooler_rubik.animate(frame_input.accumulated_time as f32);
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
