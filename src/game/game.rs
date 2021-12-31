use super::balls;
use super::level;
use super::rod;
use crate::{
    config::{SCREEN_H, SCREEN_W},
    input::Input,
    state::{Event, GameState},
    utils::return_if_some,
};
use macroquad::prelude::*;

pub const DARKGRAY_SHADOW: Color = Color::new(0.1, 0.1, 0.1, 0.5);

pub fn update_game(game: &mut GameState, input: &Input) -> Option<Event> {
    update_camera(game);

    rod::update_actuators(&mut game.objects.actuators, input);
    rod::update_rod_physics(&mut game.objects.balls, &game.objects.actuators);
    return_if_some!(level::update_level(game));
    balls::update_balls(&mut game.objects.balls);
    None
}

pub fn update_camera(game: &GameState) {
    let scale = (screen_width() / screen_height()) / (SCREEN_W / SCREEN_H);
    let (w, h) = if scale >= 1.0 {
        (SCREEN_W * scale, SCREEN_H)
        // (config::SCREEN_H * scale, config::SCREEN_W)
    } else {
        (SCREEN_W, SCREEN_H / scale)
        // (config::SCREEN_H, config::SCREEN_W / scale)
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
    level::draw_background(game);
    balls::draw_balls(&game.objects.balls, true);
    level::draw_holes(game);
    rod::draw_rod(&game.objects.actuators, false);
    balls::draw_balls(&game.objects.balls, false);
}
