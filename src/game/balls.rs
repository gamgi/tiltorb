use std::f32::consts::PI;

use crate::config::SCALE;
use crate::{game::game::DARKGRAY_SHADOW, resources::Resources, state::Ball};
use macroquad::{experimental::collections::storage, math::Vec3, prelude::*};
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

pub fn draw_balls(balls: &Vec<Ball>, rod_angle: f32) {
    let resources = storage::get_mut::<Resources>();
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
        // Ball
        draw_texture_ex(
            resources.ball_bg,
            (ball.pos.x - BALL_RADIUS) * SCALE,
            (ball.pos.y - BALL_RADIUS) * SCALE,
            WHITE,
            DrawTextureParams {
                dest_size: Some(Vec2::new(BALL_RADIUS, BALL_RADIUS) * 2. * SCALE),
                ..Default::default()
            },
        );
        draw_texture_ex(
            resources.ball_fg,
            (ball.pos.x - BALL_RADIUS) * SCALE,
            (ball.pos.y - BALL_RADIUS) * SCALE,
            WHITE,
            DrawTextureParams {
                dest_size: Some(Vec2::new(BALL_RADIUS, BALL_RADIUS) * 2. * SCALE),
                rotation: rod_angle,
                ..Default::default()
            },
        );
        // Hole shadow
        if ball.in_hole.is_some() {
            let r = (ball.pos.z).clamp(-BALL_RADIUS, 0.) / (-BALL_RADIUS);
            let shadow_r = BALL_RADIUS * ((r * PI / 2.).sin());
            draw_poly_lines(
                ball.pos.x * SCALE,
                ball.pos.y * SCALE,
                20,
                (BALL_RADIUS - shadow_r / 2.) * SCALE + 1.,
                0.,
                shadow_r * SCALE + 2.,
                Color::from_rgba(40, 40, 40, 255),
            );
            draw_poly_lines(
                ball.pos.x * SCALE,
                ball.pos.y * SCALE,
                20,
                (BALL_RADIUS - shadow_r / 2.) * SCALE + 1.,
                360. / 20. / 2.,
                shadow_r * SCALE + 2.,
                Color::from_rgba(40, 40, 40, 255),
            );
        }
    }
}
