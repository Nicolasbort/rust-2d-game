mod color;

#[derive(Debug, Clone)]
pub struct Tile {
    pub color: [f32; 4],
}

impl Tile {
    pub fn empty() -> Self {
        Tile {
            color: color::WHITE,
        }
    }

    pub fn wall() -> Self {
        Tile {
            color: color::BLACK,
        }
    }
}
