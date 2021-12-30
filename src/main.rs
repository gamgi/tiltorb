#![feature(async_closure)]

use macroquad::experimental::{collections::storage, coroutines::start_coroutine};
use macroquad::prelude::*;

mod config;
mod debug;
mod game;
mod input;
mod resources;
mod state;
mod transition;
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
    while let Some(ev) = event.take() {
        state = state.transition(ev);
        if let State::Terminating = state {
            break;
        }
        run_with_transition(&mut state, true).await?;
        event = run(&mut state).await?;
        run_with_transition(&mut state, false).await?;
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
                update(state).await?;
                next_frame().await;
            }
            Ok(Some(Event::Loaded))
        }
        State::Menu(_, _) => loop {
            return_if_some!(update(state).await?);
            next_frame().await;
        },
        State::Game(_) => loop {
            return_if_some!(update(state).await?);
            next_frame().await
        },
        _ => Ok(None),
    }
}

async fn run_with_transition(state: &mut State, out: bool) -> Result<Option<Event>> {
    let start = get_time();
    let mut event = None;
    let mut elapsed = 0.;
    while elapsed < 1. {
        event = update(state).await?.or(event);
        elapsed = transition::elapsed(start);
        transition::draw_transition(elapsed, out);
        next_frame().await;
    }
    Ok(event)
}

async fn update(state: &mut State) -> Result<Option<Event>> {
    match state {
        State::Loading => {
            clear_background(BLACK);
            draw_text("Loading", 30.0, 200.0, 30.0, WHITE);
            Ok(None)
        }
        State::Menu(game, menu) => {
            // Update
            let input = input::update_input();
            game::game::update_camera(game);
            return_if_some!(game::menu::update_menu(menu, &input));

            // Draw
            clear_background(BLACK);
            game::menu::draw_menu(&menu);
            Ok(None)
        }
        State::Game(game) => {
            // Update
            let input = input::update_input();
            let debug = game::game::update_game(game, &input);

            // Draw
            clear_background(WHITE);
            game::game::draw_game(&game);
            debug::draw_debug(&debug);
            next_frame().await
        },
        _ => Ok(None),
    }
}
