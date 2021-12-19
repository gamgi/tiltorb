use crate::input::Input;
use crate::physics;
use crate::state::GameState;
use crate::{config, resources::Resources};
use macroquad::experimental::collections::storage;
use macroquad::prelude::*;

// TODO update_state
pub fn update_game(game: &mut GameState, input: &Input) {
    update_camera(game);
    // Actuators
    physics::update_actuators(&mut game.objects.actuators, input);
    // Balls
    physics::update_balls(&mut game.objects.balls, &game.objects.actuators);
    // state
}

pub fn draw_game(game: &GameState) {
    draw_level();
    // Balls
    for ref ball in game.objects.balls.iter() {
        draw_circle(ball.pos.x, ball.pos.y, 15.0, BLUE);
    }
    // Actuators
    for ref actuator in game.objects.actuators.iter() {
        draw_rectangle(actuator.pos.x, actuator.pos.y, 60.0, 60.0, GREEN);
    }
    // Seesaw
    draw_line(
        game.objects.actuators[0].pos.x,
        game.objects.actuators[0].pos.y,
        game.objects.actuators[1].pos.x,
        game.objects.actuators[1].pos.y,
        15.0,
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

fn draw_level() {
    let resources = storage::get_mut::<Resources>();
    let aspect = resources.background.width() / resources.background.height();

    draw_texture_ex(
        resources.background,
        0.0,
        0.0,
        WHITE,
        DrawTextureParams {
            dest_size: Some(vec2(config::SCREEN_W, config::SCREEN_H / aspect)),
            ..Default::default()
        },
    );
}
