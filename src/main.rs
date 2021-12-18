use macroquad::prelude::*;
mod config;
mod input;
mod level;
mod objects;
mod state;
use crate::config as conf;
use crate::{config::window_conf, state::State};

#[macroquad::main(window_conf)]
async fn main() {
    let camera = conf::camera_conf();
    let mut state = State::new();
    set_camera(&camera);
    loop {
        clear_background(WHITE);

        let input = input::update_input();
        objects::update_objects(&mut state, &input);
        level::draw_level();
        objects::draw_objects(&state);

        next_frame().await
    }
}
