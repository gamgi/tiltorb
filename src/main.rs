#![feature(async_closure)]

use macroquad::experimental::{collections::storage, coroutines::start_coroutine};
use macroquad::prelude::*;
mod config;
mod game;
mod input;
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
        } else if let State::Terminating = state {
            break;
        } else {
            next_frame().await
        }
    }
    Ok::<(), _>(())
}

macro_rules! return_if_some {
    ( $e:expr ) => {
        match $e {
            Some(v) => {
                return Ok(Some(v));
            }
            None => {}
        }
    };
}

async fn run(state: &mut State) -> Result<Option<Event>> {
    match state {
        State::Loading => {
            let resources_future = start_coroutine(async move {
                let resources = resources::Resources::new().await.expect("Failed to load");
                storage::store(resources);
            });

            while resources_future.is_done() == false {
                clear_background(BLACK);
                draw_text("Loading", 30.0, 200.0, 30.0, WHITE);
                next_frame().await;
            }
            Ok(Some(Event::Loaded))
        }
        State::Menu(_, menu) => {
            loop {
                // Update
                let input = input::update_input();
                return_if_some!(game::menu::update_menu(menu, &input));

                // Draw
                clear_background(BLACK);
                game::menu::draw_menu(&menu);
                next_frame().await
            }
            Ok(None)
        }
        State::Game(game) => loop {
            // Update
            let input = input::update_input();
            game::game::update_game(game, &input);

            // Draw
            clear_background(WHITE);
            game::game::draw_game(&game);
            next_frame().await
        },
        _ => Ok(None),
    }
}
