use crate::{config, resources::Resources, state::State};
use macroquad::experimental::collections::storage;
use macroquad::prelude::*;

pub fn update_camera(state: &State) {
    match state {
        State::Menu(ref game) | State::Game(ref game) => {
            set_camera(&Camera2D::from_display_rect(Rect::new(
                game.camera.x,
                game.camera.y,
                config::SCREEN_W,
                config::SCREEN_H,
            )));
        }
        _ => {}
    }
}
pub fn draw_level() {
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
