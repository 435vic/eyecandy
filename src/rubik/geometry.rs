use three_d_asset::PbrMaterial;
use three_d_asset::{TriMesh, Positions};
use three_d::{CpuModel, Mat4, Srgba, Vec2, Vec3};
use cgmath::SquareMatrix;
use three_d_asset::Primitive;
use three_d_asset::Geometry;

const FACES: [Srgba; 6] = [
    Srgba::new(31, 68, 166, 255), // blue
    Srgba::new(248, 214, 73, 255), //yellow
    Srgba::new(167, 41, 55, 255), // red
    Srgba::new(255, 255, 255, 255), // white
    Srgba::new(70, 152, 81, 255), // green
    Srgba::new(235, 99, 45, 255), // orange
];

/// Same as CpuMesh::cube() but the order of faces is changed to match my internal order
/// makes it easier to compute colors for the faces and also cube turns.
pub fn cube_mesh() -> TriMesh {
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

    let mut vec_colors = vec![Srgba::WHITE; 36];
    for i in 0..6 {
        let face_color = FACES[i];
        for j in 0..6 {
            vec_colors[i * 6 + j] = face_color;
        }
    }

    let mut mesh = TriMesh {
        positions: Positions::F32(positions),
        uvs: Some(uvs),
        colors: Some(vec_colors),
        ..Default::default()
    };

    mesh.compute_normals();
    mesh.compute_tangents();
    mesh
}

/// Create a list of meshes representing a Rubik's Cube.
/// 
/// `scale` is the scaling factor for each cube.
pub fn rubik_mesh(scale: f32) -> Vec<TriMesh> {
    let offset = Mat4::from_translation(Vec3::new(-1.0, -1.0, -1.0));
    (0..27).map(|i| {
        let mut mesh = cube_mesh();
        mesh.transform(&Mat4::from_scale(0.5)).unwrap();
        mesh.transform(&offset).unwrap();
        mesh.transform(&Mat4::from_scale(scale)).expect("Scale value should be a float between 0 and 1");
        mesh.transform(
            &Mat4::from_translation(Vec3::new((i%3) as f32, ((i/3)%3) as f32, (i/9) as f32))
        ).expect("Transform should be valid");
        mesh
    }).collect()
} 

pub fn rubik_model(scale: f32) -> CpuModel {
    CpuModel {
        geometries: rubik_mesh(scale).into_iter().enumerate().map(|(i, m)| Primitive {
            name: format!("Cubelet {}", i),
            transformation: Mat4::identity(),
            animations: vec![],
            geometry: Geometry::Triangles(m),
            material_index: None
        }).collect(),
        name: String::from("Rubik"),
        materials: vec![PbrMaterial::default()]
    }
}
