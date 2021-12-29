#[derive(Debug, Clone)]
pub struct Point {
    pub y: f64,
    pub x: f64,
}

impl Default for Point {
    fn default() -> Point {
        Point { x: 0.0, y: 0.0 }
    }
}

impl Point {
    pub fn new(x: f64, y: f64) -> Self {
        Point { x, y }
    }
}
