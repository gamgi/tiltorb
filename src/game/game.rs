use super::balls;
use super::help;
use super::level;
use super::rod;
use crate::{
    config::{SCREEN_H, SCREEN_W},
    input::Input,
    state::{Event, GameState},
    utils::return_if_some,
};
use macroquad::prelude::*;

pub const DARKGRAY_SHADOW: Color = Color::new(0., 0., 0., 0.5);

pub fn update_game(game: &mut GameState, input: &Input, dt: f32) -> Option<Event> {
    update_camera(game);

    help::update_help(game, input);
    rod::update_actuators(&mut game.objects.actuators, input, dt);
    rod::update_rod_physics(&mut game.objects.balls, &game.objects.actuators);
    return_if_some!(level::update_level(game));
    balls::update_balls(&mut game.objects.balls, dt);
    None
}

pub fn update_camera(game: &GameState) {
    let scale = (screen_width() / screen_height()) / (SCREEN_W / SCREEN_H);
    let (w, h) = if scale >= 1.0 {
        (SCREEN_W * scale, SCREEN_H)
    } else {
        (SCREEN_W, SCREEN_H / scale)
    };
    let target = vec2(SCREEN_W / 2., h / 2.);
    set_camera(&Camera2D {
        target,
        zoom: vec2(1.0 / w * 2.0, -1.0 / h * 2.0),
        offset: vec2(0., 0.),
        rotation: game.camera.rotation / 3.14 * 180.0,
        render_target: None,
        viewport: None,
    });
}

pub fn draw_game(game: &GameState) {
    let rod_angle = rod::rod_angle(&game.objects.actuators);
    level::draw_background(game);
    level::draw_holes(game);
    balls::draw_balls(&game.objects.balls, rod_angle);
    rod::draw_rod(&game.objects.actuators, rod_angle);
    help::draw_help(game);
}
