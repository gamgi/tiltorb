use crate::state::GameState;
use crate::{config, config::SCALE, resources::Resources};
use macroquad::experimental::collections::storage;
use macroquad::prelude::*;

pub fn update_level(game: &mut GameState) {
    update_camera(game);
}

fn update_camera(game: &GameState) {
    set_camera(&Camera2D::from_display_rect(Rect::new(
        game.camera.x,
        game.camera.y,
        config::SCREEN_W,
        config::SCREEN_H,
    )));
}

pub fn draw_level(game: &GameState) {
    let resources = storage::get_mut::<Resources>();
    let aspect = resources.background.width() / resources.background.height();
    // Background
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
    // Holes
    for hole in &game.level.holes {
        draw_circle(hole.pos.x * SCALE, hole.pos.y * SCALE, hole.radius * SCALE, RED);
    }
}
