use crate::config::SCALE;
use crate::{game::game::DARKGRAY_SHADOW, state::Ball};
use macroquad::{math::Vec3, prelude::*};
pub const BALL_RADIUS: f32 = 0.03;

pub fn update_balls(balls: &mut Vec<Ball>, dt: f32) {
    for ball in balls.iter_mut() {
        if !ball.active {
            continue;
        }
        let forces: Vec3 = ball.forces.iter().sum();
        let impulses: Vec3 = ball.impulses.iter().sum();
        ball.impulses.clear();

        ball.vel += forces * dt + impulses;
        ball.pos += ball.vel * dt;
    }
}

pub fn draw_balls(balls: &Vec<Ball>, shadow: bool) {
    // pub fn draw_balls(game: &GameState, shadow: bool) {
    // Balls
    for ref ball in balls.iter() {
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
}
