use super::level;
use crate::debug::DebugData;
use crate::input::Input;
use crate::physics;
use crate::state::GameState;
use crate::{config, config::SCALE};
use macroquad::prelude::*;

pub const BALL_RADIUS: f32 = 0.03;
pub const ROD_RADIUS: f32 = 0.008;

pub fn update_game(game: &mut GameState, input: &Input) -> Vec<DebugData> {
    let mut debug = Vec::new();

    physics::update_actuators(&mut game.objects.actuators, input);

    debug.extend(physics::update_rod_physics(
        &mut game.objects.balls,
        &game.objects.actuators,
    ));
    debug.extend(level::update_level(game));

    physics::update_balls(&mut game.objects.balls);
    debug
}

pub fn draw_game(game: &GameState) {
    level::draw_level(game);
    // Balls
    for ref ball in game.objects.balls.iter() {
        draw_circle(
            ball.pos.x * SCALE,
            ball.pos.y * SCALE,
            BALL_RADIUS * SCALE,
            BLUE,
        );
    }
    // Actuators
    for ref actuator in game.objects.actuators.iter() {
        draw_rectangle(
            actuator.pos.x * SCALE,
            actuator.pos.y * SCALE,
            60.0,
            60.0,
            GREEN,
        );
    }
    // Seesaw
    draw_line(
        game.objects.actuators[0].pos.x * SCALE,
        game.objects.actuators[0].pos.y * SCALE,
        game.objects.actuators[1].pos.x * SCALE,
        game.objects.actuators[1].pos.y * SCALE,
        ROD_RADIUS * 2.0 * SCALE,
        BLUE,
    );
}

fn update_camera(game: &GameState) {
    set_camera(&Camera2D::from_display_rect(Rect::new(
        game.camera.x,
        game.camera.y,
        config::SCREEN_W,
        config::SCREEN_H,
    )));
}
