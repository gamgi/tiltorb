#![feature(async_closure)]

use macroquad::experimental::{collections::storage, coroutines::start_coroutine};
use macroquad::prelude::*;

mod config;
mod debug;
mod display;
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
    resources::Resources,
    state::{Event, State},
    utils::return_ok_if_some,
};
use std::error::Error;

pub type Result<T> = ::std::result::Result<T, Box<dyn Error>>;
const TARGET_DELTATIME: f32 = 0.008;

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
        State::Splash => {
            let start = get_time();
            while get_time() - start < 1. {
                draw(state);
                next_frame().await;
            }
            Ok(Some(Event::SplashTimeout))
        }
        State::Loading => {
            let resources_future = start_coroutine(async move {
                let resources = resources::Resources::new().await.expect("Failed to load");
                storage::store(resources);
            });
            storage::store(Vec::<DebugData>::new());

            while resources_future.is_done() == false {
                update(state).await?;
                draw(state);
                next_frame().await;
            }
            Ok(Some(Event::AppLoaded))
        }
        State::Menu(_, _) | State::Game(_, _) | State::Editor(_, _) => loop {
            return_ok_if_some!(update(state).await?);
            draw(state);
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
        draw(state);
        transition::draw_transition(elapsed, out);
        next_frame().await;
    }
    Ok(event)
}

async fn update(state: &mut State) -> Result<Option<Event>> {
    let (frames, dt) = calculate_frames(state);
    for _ in 0..frames {
        let input = input::update_input();
        match state {
            State::Menu(game, menu) => {
                game::game::update_camera(game);
                return_ok_if_some!(game::menu::update_menu(menu, &input));
            }
            State::Game(game, display) => {
                if input.escape {
                    return Ok(Some(Event::GameEnded));
                }
                display::update_display(display);
                return_ok_if_some!(game::game::update_game(game, &input, dt));
            }
            State::Editor(game, editor) => {
                game::game::update_camera(game);
                return_ok_if_some!(editor::update_editor(game, editor));
            }
            _ => {}
        }
    }
    Ok(None)
}

fn calculate_frames(state: &State) -> (i32, f32) {
    let dt = get_frame_time();
    match state {
        State::Game(_,_) => {
            // divide frame into substeps to improve physics collision handling
            let substeps = i32::max(1, (dt / TARGET_DELTATIME).ceil() as i32);
            (substeps, dt / substeps as f32)
        }
        _ => (1, dt),
    }
}

fn draw(state: &State) {
    match state {
        State::Loading => {
            clear_background(BLACK);
            draw_text("Loading", 30.0, 200.0, 30.0, WHITE);
        }
        State::Splash => {
            clear_background(BLACK);
            let resources = storage::get_mut::<Resources>();
            draw_texture(
                resources.splash,
                (screen_width() - resources.splash.width()) / 2.,
                (screen_height() - resources.splash.height()) / 2.,
                WHITE,
            );
        }
        State::Menu(_, menu) => {
            clear_background(BLACK);
            game::menu::draw_menu(&menu);
        }
        State::Game(game, display) => {
            clear_background(BLACK);
            game::game::draw_game(&game);
            debug::draw_debug();
            display::draw_display(&game, &display);
        }
        State::Editor(game, editor) => {
            clear_background(BLACK);
            editor::draw_editor(&game, &editor);
        }
        _ => {}
    }
}
