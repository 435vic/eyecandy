pub use graphics::run;
use three_d::{ColorMaterial, Geometry, Gm, Mat4, Mesh, Object, Srgba, Vec3};
mod graphics;

const FACES: [Color; 6] = [
    Color::Blue,
    Color::Yellow,
    Color::Red,
    Color::White,
    Color::Green,
    Color::Orange,
];

const COLORS: [Srgba; 6] = [
    Srgba::new(31, 68, 166, 255), // blue
    Srgba::new(248, 214, 73, 255), //yellow
    Srgba::new(167, 41, 55, 255), // red
    Srgba::new(255, 255, 255, 255), // white
    Srgba::new(70, 152, 81, 255), // green
    Srgba::new(235, 99, 45, 255), // orange
];

const FACELETS: [usize; 54] = [
     0,  1,  2,  3,  4,  5,  6,  7,  8,
     0,  9, 18,  1, 10, 19,  2, 11, 20,
     2, 11, 20,  5, 14, 23,  8, 17, 26,
     8, 17, 26,  7, 16, 25,  6, 15, 24,
    20, 19, 18, 23, 22, 21, 26, 25, 24,
    18,  9,  0, 21, 12,  3, 24, 15,  6
];

// const CORNERS: [i32; 8] = [0, 2, 6, 8, 18, 20, 24, 26];
// const EDGES: [i32; 12] = [1, 3, 5, 7, 9, 11, 15, 17, 19, 21, 23, 25];
// const CENTERS: [i32; 6] = [4, 10, 12, 14, 16, 22];

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Color {
    Blue,
    Yellow,
    Red,
    White,
    Green,
    Orange,
    None
}

impl Into<Srgba> for Color {
    fn into(self) -> Srgba {
        match self {
            Color::Blue => COLORS[0],
            Color::Yellow => COLORS[1],
            Color::Red => COLORS[2],
            Color::White => COLORS[3],
            Color::Green => COLORS[4],
            Color::Orange => COLORS[5],
            Color::None => Srgba::BLACK,
        }
    }
}

impl Default for Color {
    fn default() -> Self {
        Color::None
    }
}

type PieceMaterial = ColorMaterial;

pub(crate) struct Piece {
    position: (i32, i32, i32),
    color: (Color, Color, Color),
    gm: Gm<Mesh, PieceMaterial>
}

pub struct Cube {
    pub(crate) pieces: Vec<Piece>,
}

impl Cube {
    pub fn from_facelet_str(fstr: String, ctx: &three_d::Context) -> Result<Cube, String> {
        let mut pieces = vec![];
        for i in 0..27 {
            let position = (i as i32 / 9 - 1, 2 - (i as i32 / 3) % 3 - 1, i as i32 % 3 - 1);
            let mut mesh = graphics::cube_mesh();
            mesh.transform(&Mat4::from_scale(0.5)).unwrap();
            mesh.transform(
                &Mat4::from_translation(Vec3::new(position.0 as f32, position.1 as f32, position.2 as f32))
            ).unwrap();
            pieces.push((position, (Color::None, Color::None, Color::None)));
        }
        for facelet in 0..54 {
            let cubelet = FACELETS[facelet];
            let color = match fstr.chars().nth(facelet) {
                Some('B') => Color::Blue,
                Some('Y') => Color::Yellow,
                Some('R') => Color::Red,
                Some('W') => Color::White,
                Some('G') => Color::Green,
                Some('O') => Color::Orange,
                Some(c) => return Err(format!("Invalid char {}", c)),
                None => return Err("Invalid string length".to_string())
            };
            match facelet / 9 {
                0 | 4 => {
                    pieces[cubelet].1.0 = color;
                },
                1 | 3 => {
                    pieces[cubelet].1.1 = color;
                },
                2 | 5 => {
                    pieces[cubelet].1.2 = color;
                },
                _ => { return Err(format!("Error in facelet conversion: {} / 9 = {}", facelet, facelet/9)) }
            }
        }
        let pieces = pieces.into_iter().enumerate().map(|(i, (position, color))| {
            println!("{}: {:?} {:?}", i, position, color);
            let mut mesh = graphics::cube_mesh();
            mesh.transform(&Mat4::from_scale(0.5)).unwrap();
            mesh.transform(&Mat4::from_scale(0.98)).unwrap();
            mesh.transform(
                &Mat4::from_translation(Vec3::new(position.0 as f32, position.1 as f32, position.2 as f32))
            ).unwrap();
            let mut face_colors = vec![Srgba::BLACK; 36];
            for i in 0..6 {
                let face_color = match i {
                    0 => if position.0 == -1 { color.0 } else { Color::None },
                    1 => if position.1 ==  1 { color.1 } else { Color::None },
                    2 => if position.2 ==  1 { color.2 } else { Color::None },
                    3 => if position.1 == -1 { color.1 } else { Color::None },
                    4 => if position.0 ==  1 { color.0 } else { Color::None },
                    5 => if position.2 == -1 { color.2 } else { Color::None },
                    _ => Color::None,
                };
                for j in 0..6 {
                    face_colors[i * 6 + j] = face_color.into();
                }
            }
            mesh.colors = Some(face_colors);
            Piece {
                position,
                color,
                gm: Gm::new(Mesh::new(ctx, &mesh), PieceMaterial::default())
            }
        }).collect::<Vec<_>>();
        Ok(Cube { pieces })
    }

    pub fn solved(ctx: &three_d::Context) -> Cube {
        Self::from_facelet_str("BBBBBBBBBYYYYYYYYYRRRRRRRRRWWWWWWWWWGGGGGGGGGOOOOOOOOO".to_string(), ctx).unwrap()
    }
}

impl Geometry for Piece {
    fn aabb(&self) -> three_d::prelude::AxisAlignedBoundingBox {
        self.gm.aabb()
    }

    fn animate(&mut self, _time: f32) {
        self.gm.animate(_time)
    }

    fn draw(
            &self,
            camera: &three_d::Camera,
            program: &three_d::Program,
            render_states: three_d::RenderStates,
            attributes: three_d::FragmentAttributes,
        ) {
        self.gm.draw(camera, program, render_states, attributes)
    }

    fn id(&self, required_attributes: three_d::FragmentAttributes) -> u16 {
        self.gm.id(required_attributes)
    }

    fn render_with_effect(
            &self,
            material: &dyn three_d::Effect,
            camera: &three_d::Camera,
            lights: &[&dyn three_d::Light],
            color_texture: Option<three_d::ColorTexture>,
            depth_texture: Option<three_d::DepthTexture>,
        ) {
        self.gm.render_with_effect(material, camera, lights, color_texture, depth_texture)
    }

    fn render_with_material(&self, material: &dyn three_d::Material, camera: &three_d::Camera, lights: &[&dyn three_d::Light]) {
        self.gm.render_with_material(material, camera, lights)
    }

    fn vertex_shader_source(&self, required_attributes: three_d::FragmentAttributes) -> String {
        self.gm.vertex_shader_source(required_attributes)
    }
}

impl Object for Piece {
    fn material_type(&self) -> three_d::MaterialType {
        self.gm.material_type()
    }

    fn render(&self, camera: &three_d::Camera, lights: &[&dyn three_d::Light]) {
        self.gm.render(camera, lights)
    }
}

impl<'a> IntoIterator for &'a Piece {
    type Item = &'a dyn Object;
    type IntoIter = std::vec::IntoIter<&'a dyn Object>;

    fn into_iter(self) -> Self::IntoIter {
        vec![self as &dyn Object].into_iter()
    }
}

impl<'a> IntoIterator for &'a Cube {
    type Item = &'a dyn Object;
    type IntoIter = std::vec::IntoIter<&'a dyn Object>;

    fn into_iter(self) -> Self::IntoIter {
        self.pieces.iter()
            .map(|p| p as &dyn Object)
            .collect::<Vec<_>>()
            .into_iter()
    }
}

