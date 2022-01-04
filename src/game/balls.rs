use std::f32::consts::PI;

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

pub fn draw_balls(balls: &Vec<Ball>) {
    for ref ball in balls.iter() {
        // Wall shadow
        let ball_shadow_pos = ball.pos.truncate()
            + Vec2::new(
                f32::max(0., ball.pos.z * 0.5),
                f32::max(0., ball.pos.z * 0.1),
            );
        draw_circle(
            ball_shadow_pos.x * SCALE,
            ball_shadow_pos.y * SCALE,
            BALL_RADIUS * SCALE,
            DARKGRAY_SHADOW,
        );
        if ball.in_hole.is_some() {
            // Hole shadow
            let r = (ball.pos.z).clamp(-BALL_RADIUS, 0.) / (-BALL_RADIUS);
            let shadow_r = BALL_RADIUS * (1. - (r * PI / 2.).sin());
            draw_circle(
                ball.pos.x * SCALE,
                ball.pos.y * SCALE,
                BALL_RADIUS * SCALE,
                DARKGRAY,
            );
            // Ball
            draw_circle(
                ball.pos.x * SCALE,
                ball.pos.y * SCALE,
                shadow_r * SCALE,
                LIGHTGRAY,
            );
        } else {
            // Ball
            draw_circle(
                ball.pos.x * SCALE,
                ball.pos.y * SCALE,
                BALL_RADIUS * SCALE,
                LIGHTGRAY,
            );
        }
    }
}
