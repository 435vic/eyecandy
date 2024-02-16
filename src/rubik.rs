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
