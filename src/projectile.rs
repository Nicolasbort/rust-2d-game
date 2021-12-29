use opengl_graphics::GlGraphics;
use piston::input::{Button, Key, RenderArgs};

#[path = "./color.rs"]
mod color;
#[path = "./point.rs"]
mod point;
#[path = "./size.rs"]
mod size;

use point::Point;

#[derive(Debug)]
pub struct Projectile {
    pub size: size::Size,
    pub speed: f64,
    pub color: [f32; 4],
    pub position: Point,
    pub direction: i8
}

impl Projectile {
    pub fn new(x: f64, y: f64, direction: i8) -> Self {
        Projectile {
            size: size::Size::new(5, 5),
            color: color::BLUE,
            speed: 5.0,
            position: Point::new(x, y),
            direction,
        }
    }

    pub fn render(&self, gl: &mut GlGraphics, args: &RenderArgs) {
        use graphics::rectangle;

        gl.draw(args.viewport(), |c, gl| {
            rectangle(
                self.color,
                rectangle::square(self.position.x, self.position.y, self.size.width as f64),
                c.transform,
                gl,
            );
        });
    }
}