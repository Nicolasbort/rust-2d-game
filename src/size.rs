#[derive(Debug, Clone)]
pub struct Size {
    pub height: u8,
    pub width: u8,
}

impl Size {
    pub fn new(width: u8, height: u8) -> Self {
        Size { width, height }
    }
}
