use super::balls;
use super::level;
use super::rod;
use crate::config::SCALE;
use crate::debug::DebugData;
use crate::input::Input;
use crate::state::GameState;
use macroquad::prelude::*;

pub const DARKGRAY_SHADOW: Color = Color::new(0.31, 0.31, 0.31, 0.8);

pub fn update_game(game: &mut GameState, input: &Input) -> Vec<DebugData> {
    let mut debug = Vec::new();

    rod::update_actuators(&mut game.objects.actuators, input);
    debug.extend(rod::update_rod_physics(
        &mut game.objects.balls,
        &game.objects.actuators,
    ));
    debug.extend(level::update_level(game));
    debug.extend(balls::update_balls(&mut game.objects.balls));
    debug
}

pub fn draw_game(game: &GameState) {
    level::draw_background(game);
    balls::draw_balls(&game.objects.balls, true);
    level::draw_holes(game);
    rod::draw_rod(&game.objects.actuators, false);
    balls::draw_balls(&game.objects.balls, false);
}
