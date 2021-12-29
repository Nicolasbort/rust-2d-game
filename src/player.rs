use opengl_graphics::GlGraphics;
use piston::input::{Button, Key, RenderArgs};

mod color;
mod point;

use point::Point;

pub struct Player {
    color: [f32; 4],
    position: Point,
}

impl Player {
    pub fn new(x: f64, y: f64) -> Self {
        Player {
            color: color::RED,
            position: Point::new(x, y),
        }
    }

    pub fn render(&self, gl: &mut GlGraphics, args: &RenderArgs) {
        use graphics::rectangle;

        gl.draw(args.viewport(), |c, gl| {
            rectangle(
                self.color,
                rectangle::square(self.position.x, self.position.y, crate::PIXEL_SIZE),
                c.transform,
                gl,
            );
        });
    }

    pub fn handle_keyboard(&mut self, button: Button) {
        match button {
            Button::Keyboard(Key::Up) => self.position.y -= crate::PIXEL_SIZE,
            Button::Keyboard(Key::Down) => self.position.y += crate::PIXEL_SIZE,
            Button::Keyboard(Key::Right) => self.position.x += crate::PIXEL_SIZE,
            Button::Keyboard(Key::Left) => self.position.x -= crate::PIXEL_SIZE,
            _ => (),
        }
    }
}
