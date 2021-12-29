use opengl_graphics::GlGraphics;
use piston::input::{RenderArgs};

mod color;
mod point;
mod projectile;

use point::Point;
use projectile::Projectile;

pub struct Player {
    pub name: String,
    color: [f32; 4],
    pub position: Point,
    pub projectiles: Vec<Projectile>,
    pub projectile_direction: i8,
}

impl Player {
    pub fn new(name: &str, x: f64, y: f64, projectile_direction: i8) -> Self {
        Player {
            name: String::from(name),
            color: color::RED,
            position: Point::new(x, y),
            projectiles: vec![],
            projectile_direction,
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

    pub fn shot_projectile(&mut self) {
        println!("{} shot a projectile", self.name);

        let mut x = self.position.x;
        if self.projectile_direction > 0 {
            x += crate::PIXEL_SIZE;
        }

        self.projectiles.push(Projectile::new(
            x,
            self.position.y + (crate::PIXEL_SIZE / 2.0) - 5.0,
            self.projectile_direction,
        ));
    }

    pub fn remove_projectiles(&mut self, window_limit: &u32) {
        let mut index = 0;
        let mut index_to_remove: i16 = -1;

        for projectile in self.projectiles.iter_mut() {
            projectile.position.x += projectile.speed * projectile.direction as f64;

            if projectile.position.x < 0.0
                || projectile.position.x + projectile.size.width as f64 > *window_limit as f64
            {
                index_to_remove = index;
            }

            index += 1;
        }

        if index_to_remove != -1 {
            println!(
                "{} removing projectile. Count projectiles: {}.",
                self.name,
                self.projectiles.len()
            );
            self.projectiles.remove(index_to_remove as usize);
        }
    }
}
