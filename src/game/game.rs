use super::level;
use crate::config::SCALE;
use crate::debug::DebugData;
use crate::input::Input;
use crate::physics;
use crate::state::GameState;
use macroquad::prelude::*;

pub const BALL_RADIUS: f32 = 0.03;
pub const ROD_RADIUS: f32 = 0.008;
const DARKGRAY_SHADOW: Color = Color::new(0.31, 0.31, 0.31, 0.8);

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
    level::draw_background(game);
    draw_game_objects(game, true);
    level::draw_holes(game);
    draw_game_objects(game, false);
}

pub fn draw_game_objects(game: &GameState, shadow: bool) {
    // Balls
    for ref ball in game.objects.balls.iter() {
        if shadow {
            draw_circle(
                ball.pos.x * SCALE + f32::max(0., ball.pos.z * SCALE * 0.5),
                ball.pos.y * SCALE + f32::max(0., ball.pos.z * SCALE * 0.1),
                BALL_RADIUS * SCALE,
                DARKGRAY_SHADOW,
            );
        } else {
            draw_circle(
                ball.pos.x * SCALE,
                ball.pos.y * SCALE,
                BALL_RADIUS * SCALE,
                LIGHTGRAY,
            );
        }
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
    if shadow {
        draw_line(
            game.objects.actuators[0].pos.x * SCALE + physics::ACTUATOR_Z * SCALE * 0.5,
            game.objects.actuators[0].pos.y * SCALE + physics::ACTUATOR_Z * SCALE * 0.1,
            game.objects.actuators[1].pos.x * SCALE + physics::ACTUATOR_Z * SCALE * 0.5,
            game.objects.actuators[1].pos.y * SCALE + physics::ACTUATOR_Z * SCALE * 0.1,
            ROD_RADIUS * 2.0 * SCALE,
            DARKGRAY_SHADOW,
        );
    } else {
        draw_line(
            game.objects.actuators[0].pos.x * SCALE,
            game.objects.actuators[0].pos.y * SCALE,
            game.objects.actuators[1].pos.x * SCALE,
            game.objects.actuators[1].pos.y * SCALE,
            ROD_RADIUS * 2.0 * SCALE,
            LIGHTGRAY,
        );
    }
}
