#![feature(async_closure)]

use macroquad::experimental::{collections::storage, coroutines::start_coroutine};
use macroquad::prelude::*;

mod config;
mod debug;
mod editor;
mod game;
mod input;
mod resources;
mod state;
mod transition;
mod utils;
use crate::{
    config::window_conf,
    debug::DebugData,
    state::{Event, State},
    utils::return_ok_if_some,
};
use std::error::Error;

pub type Result<T> = ::std::result::Result<T, Box<dyn Error>>;

#[macroquad::main(window_conf)]
async fn main() -> Result<()> {
    let mut state = State::Initial;
    let mut event = Some(Event::AppInitialized);
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

async fn run(state: &mut State) -> Result<Option<Event>> {
    match state {
        State::Loading => {
            let resources_future = start_coroutine(async move {
                let resources = resources::Resources::new().await.expect("Failed to load");
                storage::store(resources);
            });
            storage::store(Vec::<DebugData>::new());

            while resources_future.is_done() == false {
                update(state).await?;
                next_frame().await;
            }
            Ok(Some(Event::AppLoaded))
        }
        State::Menu(_, _) | State::Game(_) | State::Editor(_, _) => loop {
            return_ok_if_some!(update(state).await?);
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
        }
        State::Menu(game, menu) => {
            // Update
            let input = input::update_input();
            game::game::update_camera(game);
            return_ok_if_some!(game::menu::update_menu(menu, &input));

            // Draw
            clear_background(BLACK);
            game::menu::draw_menu(&menu);
        }
        State::Game(game) => {
            // Update
            let input = input::update_input();
            if input.escape {
                return Ok(Some(Event::GameEnded));
            }
            return_ok_if_some!(game::game::update_game(game, &input));

            // Draw
            clear_background(WHITE);
            game::game::draw_game(&game);
            // debug::draw_debug();
        }
        State::Editor(game, editor) => {
            // Update
            game::game::update_camera(game);
            return_ok_if_some!(editor::update_editor(game, editor));

            // Draw
            clear_background(WHITE);
            editor::draw_editor(&game, &editor);
        }
        _ => {}
    }
    Ok(None)
}
