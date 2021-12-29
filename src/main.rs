extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

mod game;
use game::Game;

use piston::event_loop::{EventSettings, Events};
use piston::input::{ButtonEvent, ButtonState, RenderEvent, UpdateEvent};
use piston::EventLoop;

const WINDOW_SIZE: u32 = 512;
const PIXEL_SIZE: f64 = 32.0;

fn main() {
    let mut game = Game::new(WINDOW_SIZE, PIXEL_SIZE);

    let mut events = Events::new(EventSettings::new()).ups(8);
    while let Some(e) = events.next(&mut game.window) {
        if let Some(args) = e.render_args() {
            game.render(&args);
        }

        if let Some(args) = e.update_args() {
            game.update(&args);
        }
        if let Some(k) = e.button_args() {
            if k.state == ButtonState::Press {
                game.player.handle_keyboard(k.button);
            }
        }
    }
}
