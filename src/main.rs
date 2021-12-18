use macroquad::experimental::collections::storage;
use macroquad::prelude::*;
mod config;
mod input;
mod level;
mod objects;
mod physics;
mod resources;
mod state;
use crate::config as conf;
use crate::{config::window_conf, state::State};
use std::error::Error;

pub type Result<T> = ::std::result::Result<T, Box<dyn Error>>;

#[macroquad::main(window_conf)]
async fn main() -> Result<()> {
    let mut state = State::new();
    storage::store(resources::Resources::new().await?);
    loop {
        clear_background(WHITE);

        let input = input::update_input();
        objects::update_objects(&mut state, &input);
        level::draw_level();
        objects::draw_objects(&state);

        next_frame().await
    }
}
