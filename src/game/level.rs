use crate::{
    config,
    config::SCALE,
    game::game::BALL_RADIUS,
    resources::Resources,
    state::{Ball, GameState, Hole},
};
use macroquad::experimental::collections::storage;
use macroquad::prelude::*;

pub fn update_level(game: &mut GameState) {
    update_camera(game);

    update_balls(&mut game.objects.balls, &game.level.holes);
}

fn update_camera(game: &GameState) {
    set_camera(&Camera2D::from_display_rect(Rect::new(
        game.camera.x,
        game.camera.y,
        config::SCREEN_W,
        config::SCREEN_H,
    )));
}

pub fn update_balls(balls: &mut Vec<Ball>, holes: &Vec<Hole>) {
    let dt = get_frame_time();
    for ball in balls.iter_mut() {
        if !ball.active {
            continue;
        }
        for hole in holes {
            let distance = ball.pos.distance(hole.pos);
            if distance <= hole.radius {
                let unit_force = (hole.pos - ball.pos).normalize();
                let inverse_distance = 1.0 - (distance / (hole.radius));
                // Naive z-vector
                let multiplier = (inverse_distance * 3.14).sin() * 5.0;
                let az = unit_force * multiplier;
                ball.vel += dt * unit_force * multiplier;
            }
        }
        println!("{},{},{}", ball.vel.x, ball.zvel, ball.zpos);
    }
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
        draw_circle(
            hole.pos.x * SCALE,
            hole.pos.y * SCALE,
            hole.radius * SCALE,
            RED,
        );
    }
}
