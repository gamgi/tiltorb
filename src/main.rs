#![feature(async_closure)]

use macroquad::experimental::{collections::storage, coroutines::start_coroutine};
use macroquad::prelude::*;
mod config;
mod input;
mod level;
mod objects;
mod physics;
mod resources;
mod state;
use crate::{
    config::window_conf,
    state::{Event, State},
};
use std::error::Error;

pub type Result<T> = ::std::result::Result<T, Box<dyn Error>>;

#[macroquad::main(window_conf)]
async fn main() -> Result<()> {
    let mut state = State::Initial;
    let mut event = Some(Event::Initialized);
    loop {
        if let Some(ev) = event.take() {
            state = state.transition(ev);
            event = run(&mut state).await?;
        } else {
            next_frame().await
        }
    }
    Ok::<(), _>(())
}

async fn run(state: &mut State) -> Result<Option<Event>> {
    match *state {
        State::Loading => {
            let resources_future = start_coroutine(async move {
                let resources = resources::Resources::new().await.expect("Failed to load");
                storage::store(resources);
            });

            while resources_future.is_done() == false {
                clear_background(BLACK);
                draw_text("Loading", 30.0, 200.0, 30.0, WHITE);
                next_frame().await
            }
            Ok(Some(Event::Loaded))
        }
        State::Menu(_) => Ok(None),
        State::Game(_) => loop {
            clear_background(WHITE);
            let input = input::update_input();
            objects::update_objects(state, &input);
            level::update_camera(&state);
            level::draw_level();
            objects::draw_objects(&state);
            next_frame().await
        },
        _ => Ok(None),
    }
}
