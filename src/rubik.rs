use std::collections::VecDeque;

use cgmath::{Rad, SquareMatrix, Transform};
pub use graphics::run;
use log::{debug, trace};
use three_d::{ColorMaterial, Geometry, Gm, Mat3, Mat4, Mesh, Object, Srgba, Vec3};
mod graphics;

// const FACE_ORDER: [Color; 6] = [
//     Color::Blue,
//     Color::Yellow,
//     Color::Red,
//     Color::White,
//     Color::Green,
//     Color::Orange,
// ];

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

pub const ROT_XY_CW: Mat3 = Mat3::new(
    1.0, 0.0, 0.0,
    0.0, 0.0, 1.0,
    0.0, -1.0, 0.0
);

pub const ROT_XY_CCW: Mat3 = Mat3::new(
    1.0, 0.0, 0.0,
    0.0, 0.0, -1.0,
    0.0, 1.0, 0.0
);

pub const ROT_XZ_CW: Mat3 = Mat3::new(
    0.0, 0.0, 1.0,
    0.0, 1.0, 0.0,
    -1.0, 0.0, 0.0
);

pub const ROT_XZ_CCW: Mat3 = Mat3::new(
    0.0, 0.0, -1.0,
    0.0, 1.0, 0.0,
    1.0, 0.0, 0.0
);

pub const ROT_YZ_CW: Mat3 = Mat3::new(
    0.0, -1.0, 0.0,
    1.0, 0.0, 0.0,
    0.0, 0.0, 1.0
);

pub const ROT_YZ_CCW: Mat3 = Mat3::new(
    0.0, 1.0, 0.0,
    -1.0, 0.0, 0.0,
    0.0, 0.0, 1.0
);

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Move {
    L, LP,
    R, RP,
    U, UP,
    D, DP,
    F, FP,
    B, BP,
}

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
    transform: Mat4,
    gm: Gm<Mesh, PieceMaterial>
}

pub struct Cube {
    pub(crate) pieces: Vec<Piece>,
    current_move: Option<Move>,
    current_face: Option<[usize; 9]>,
    move_start: f32,
    move_queue: VecDeque<Move>,
    move_time: f32,
}

impl Piece {
    pub fn vec(&self) -> Vec3 {
        Vec3::new(self.position.0 as f32, self.position.1 as f32, self.position.2 as f32)
    }

    fn cubelet(&self) -> usize {
        let out = ((self.position.0 + 1) * 9 + (1 - self.position.1) * 3 + (self.position.2 + 1)).try_into().unwrap();
        out
    }

    // Rotate the piece using a matrix
    // will only affect the internal position, mesh will not be changed
    pub fn rotate(&mut self, mat: Mat3) -> Result<(Vec3, Vec3), String> {
        let prev = self.vec();
        let new = mat * prev;
        let mut rot = new - prev;
        let unchanged_axes = (0..3).filter(|&i| rot[i] == 0.0).collect::<Vec<_>>();
        if unchanged_axes.len() == 3 {
            return Ok((prev, new));
        } else if unchanged_axes.len() == 2 {
            rot += mat * rot;
        }

        if (0..3).filter(|&i| rot[i] == 0.0).count() != 1 {
            return Err(format!("before: {:?}, after: {:?}, rot:{:?}", prev, new, rot));
        }
        let diff = (0..3).into_iter().filter(|&i| rot[i] != 0.0).collect::<Vec<_>>();
        let mut color_arr = [self.color.0, self.color.1, self.color.2];
        color_arr.swap(diff[0], diff[1]);
        self.color = (color_arr[0], color_arr[1], color_arr[2]);
        self.position = (new.x as i32, new.y as i32, new.z as i32);
        Ok((prev, new))
    }

    pub fn transform(&mut self, mat: Mat4) {
        self.gm.set_transformation(mat * self.transform);
    }
}

impl Move {
    pub fn face(&self) -> usize {
        match self {
            Move::L | Move::LP => 0,
            Move::U | Move::UP => 1,
            Move::F | Move::FP => 2,
            Move::D | Move::DP => 3,
            Move::R | Move::RP => 4,
            Move::B | Move::BP => 5,
        }
    }

    pub fn transform(&self, t: f32) -> Mat4 {
        match self {
            Move::L | Move::RP => Mat4::from_angle_x(Rad(t * std::f32::consts::FRAC_PI_2)),
            Move::LP | Move::R => Mat4::from_angle_x(Rad(-t * std::f32::consts::FRAC_PI_2)),
            Move::U | Move::DP => Mat4::from_angle_y(Rad(-t * std::f32::consts::FRAC_PI_2)),
            Move::UP | Move::D => Mat4::from_angle_y(Rad(t * std::f32::consts::FRAC_PI_2)),
            Move::F | Move::BP => Mat4::from_angle_z(Rad(-t * std::f32::consts::FRAC_PI_2)),
            Move::FP | Move::B => Mat4::from_angle_z(Rad(t * std::f32::consts::FRAC_PI_2)),
        }
    }
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
        let pieces = pieces.into_iter().enumerate().map(|(_i, (position, color))| {
            // let calculated_index = (position.0 + 1) * 9 + (1 - position.1) * 3 + (position.2 + 1);
            // println!("{} -> {}: {:?}, {:?}", _i, calculated_index, position, color);
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
                transform: Mat4::identity(),
                gm: Gm::new(Mesh::new(ctx, &mesh), PieceMaterial::default())
            }
        }).collect::<Vec<_>>();
        Ok(Cube {
            pieces,
            current_move: None,
            current_face: None,
            move_start: 0.0,
            move_queue: VecDeque::new(),
            move_time: 1000.0,
        })
    }

    // TODO: remove crate visibility
    pub(crate) fn face_iter(&mut self, face: usize) -> impl Iterator<Item=&mut Piece> {
        let face_cis: Vec<&usize> = FACELETS.iter().skip(face * 9).take(9).collect();
        self.pieces.iter_mut()
            .filter(move |p| face_cis.contains(&&p.cubelet()))
    }

    fn face(&self, face: usize) -> [usize; 9] {
        let face_cis: Vec<&usize> = FACELETS.iter().skip(face * 9).take(9).collect();
        self.pieces.iter().enumerate()
            .filter_map(|(i, p)| if face_cis.contains(&&p.cubelet()) { Some(i) } else { None })
            .collect::<Vec<_>>()
            .try_into()
            .unwrap()
    }

    pub(crate) fn _dbg_rotate_face_model(&mut self, face: usize, amt: Rad<f32>) -> Result<(), String> {
        let rot = match face {
            0 => Mat4::from_angle_x(amt),
            1 => Mat4::from_angle_y(amt),
            2 => Mat4::from_angle_z(amt),
            _ => return Err(format!("Invalid face {}", face))
        };
        for cubelet in self.face_iter(face) {
            cubelet.gm.set_transformation(rot);
        }
        Ok(())
    }

    fn rotate_face(&mut self, face: usize, mat: Mat3) {
        self.face_iter(face).for_each(|piece| {
            let prev = piece.position;
            piece.rotate(mat).unwrap();
            trace!("rot {}: {:?} -> {:?}", face, prev, piece.position);
        });
    }

    fn apply_move(&mut self, mv: Move) {
        match mv {
            Move::L => self.rotate_face(0, ROT_XY_CW),
            Move::LP => self.rotate_face(0, ROT_XY_CCW),
            Move::R => self.rotate_face(4, ROT_XY_CCW),
            Move::RP => self.rotate_face(4, ROT_XY_CW),
            Move::U => self.rotate_face(1, ROT_XZ_CW),
            Move::UP => self.rotate_face(1, ROT_XZ_CCW),
            Move::D => self.rotate_face(3, ROT_XZ_CCW),
            Move::DP => self.rotate_face(3, ROT_XZ_CW),
            Move::F => self.rotate_face(2, ROT_YZ_CW),
            Move::FP => self.rotate_face(2, ROT_YZ_CCW),
            Move::B => self.rotate_face(5, ROT_YZ_CCW),
            Move::BP => self.rotate_face(5, ROT_YZ_CW),
        }
    }

    pub fn queue(&mut self, mv: impl Iterator<Item = Move>) {
        self.move_queue.extend(mv);
    }

    pub fn animate(&mut self, time: f32) {
        if let (Some(mv), Some(cface)) = (self.current_move, self.current_face) {
            let elapsed = time - self.move_start;
            if elapsed > self.move_time {
                for ci in cface {
                    let piece = &mut self.pieces[ci];
                    piece.transform = mv.transform(1.0) * piece.transform;
                    piece.transform(Mat4::identity());
                }
                self.apply_move(mv);
                trace!("Applied move {:?}", mv);
                self.current_move = None;
            } else {
                let x = elapsed / self.move_time;
                for ci in cface {
                    let piece = &mut self.pieces[ci];
                    piece.transform(mv.transform(x));
                }
            }
        } else if let Some(nmv) = self.move_queue.pop_front() {
            self.current_move = Some(nmv);
            self.current_face = Some(self.face(nmv.face()));
            self.move_start = time;
            trace!("New move {:?} will affect {:?}", nmv, self.face_iter(nmv.face()).map(|c| c.cubelet()).collect::<Vec<_>>());
        }
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

impl <'a> IntoIterator for Move {
    type Item = Move;
    type IntoIter = std::vec::IntoIter<Move>;

    fn into_iter(self) -> Self::IntoIter {
        vec![self].into_iter()
    }
}

