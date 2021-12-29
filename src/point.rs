#[derive(Debug, Clone)]
pub struct Point {
    pub y: f64,
    pub x: f64,
}

impl Point {
    pub fn new(x: f64, y: f64) -> Self {
        Point { x, y }
    }
}
