use glutin_window::GlutinWindow as Window;
use opengl_graphics::GlGraphics;
use piston::input::{RenderArgs, UpdateArgs};
use piston::window::WindowSettings;

#[path = "./color.rs"]
mod color;
#[path = "./player.rs"]
mod player;
#[path = "./tile.rs"]
mod tile;

use player::Player;

pub struct Game {
    pub gl: GlGraphics,
    pub window: Window,
    pub player: Player,
    pub right_player: Player,
    window_size: u32,
    pixel_size: f64,
    world_size: u32,
}

impl Game {
    pub fn new(window_size: u32, pixel_size: f64) -> Self {
        use opengl_graphics::OpenGL;

        let opengl = OpenGL::V3_2;

        let window: Window = WindowSettings::new("Rust game", [window_size, window_size])
            .graphics_api(opengl)
            .exit_on_esc(true)
            .build()
            .unwrap();

        let gl = GlGraphics::new(opengl);

        Game {
            gl,
            window,
            player: Player::new("Nicolas", 0.0, 0.0, 1),
            right_player: Player::new("Joaozinho", window_size as f64 - pixel_size, 0.0, -1),
            window_size,
            pixel_size,
            world_size: window_size / pixel_size as u32,
        }
    }

    pub fn render(&mut self, args: &RenderArgs) {
        self.create_map(&args);

        for projectile in self.player.projectiles.iter_mut() {
            projectile.render(&mut self.gl, &args);
        }

        for projectile in self.right_player.projectiles.iter_mut() {
            projectile.render(&mut self.gl, &args);
        }

        self.player.render(&mut self.gl, &args);
        self.right_player.render(&mut self.gl, &args);
    }

    pub fn update(&mut self, args: &UpdateArgs) {
        self.player.remove_projectiles(&self.window_size);
        self.right_player.remove_projectiles(&self.window_size);
    }

    pub fn create_map(&mut self, args: &RenderArgs) {
        use graphics::*;

        let mut map =
            vec![vec![tile::Tile::empty(); self.world_size as usize]; self.world_size as usize];

        map[self.world_size as usize / 2][self.world_size as usize / 2] = tile::Tile::wall();

        self.gl.draw(args.viewport(), |c, gl| {
            clear(color::GREEN, gl);

            for i in 0..self.world_size {
                for j in 0..self.world_size {
                    rectangle(
                        map[i as usize][j as usize].color,
                        rectangle::square(
                            self.pixel_size * i as f64,
                            self.pixel_size * j as f64,
                            self.pixel_size,
                        ),
                        c.transform,
                        gl,
                    );
                }
            }
        });
    }
}
