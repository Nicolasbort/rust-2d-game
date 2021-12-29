extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

mod game;
use game::Game;

use piston::event_loop::{EventSettings, Events};
use piston::input::{ButtonEvent, ButtonState, RenderEvent, UpdateEvent, Button, Key};
use piston::EventLoop;

const WINDOW_SIZE: u32 = 512;
const PIXEL_SIZE: f64 = 32.0;

fn main() {
    let mut game = Game::new(WINDOW_SIZE, PIXEL_SIZE);
    let mut events = Events::new(EventSettings::new()).ups(30);

    while let Some(e) = events.next(&mut game.window) {
        if let Some(args) = e.render_args() {
            game.render(&args);
        }

        if let Some(args) = e.update_args() {
            game.update(&args);
        }

        if let Some(k) = e.button_args() {
            if k.state == ButtonState::Press {
                match k.button {
                    Button::Keyboard(Key::Up) => game.right_player.position.y -= crate::PIXEL_SIZE,
                    Button::Keyboard(Key::Down) => game.right_player.position.y += crate::PIXEL_SIZE,
                    Button::Keyboard(Key::Right) => game.right_player.position.x += crate::PIXEL_SIZE,
                    Button::Keyboard(Key::Left) => game.right_player.position.x -= crate::PIXEL_SIZE,
                    Button::Keyboard(Key::Return) => game.right_player.shot_projectile(),

                    Button::Keyboard(Key::W) => game.player.position.y -= crate::PIXEL_SIZE,
                    Button::Keyboard(Key::S) => game.player.position.y += crate::PIXEL_SIZE,
                    Button::Keyboard(Key::D) => game.player.position.x += crate::PIXEL_SIZE,
                    Button::Keyboard(Key::A) => game.player.position.x -= crate::PIXEL_SIZE,
                    Button::Keyboard(Key::Space) => game.player.shot_projectile(),
                    _ => (),
                }
            }
        }
    }
}
