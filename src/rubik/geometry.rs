use three_d::*;

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