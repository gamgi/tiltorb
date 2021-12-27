use crate::{
    config,
    config::SCALE,
    debug::DebugData,
    game::game::BALL_RADIUS,
    resources::Resources,
    state::{Ball, GameState, Hole},
};
use macroquad::experimental::collections::storage;
use macroquad::math::{Vec2, Vec3};
use macroquad::prelude::*;

enum BallState {
    Inside,
    InsideEdge,
    OutsideEdge,
    Outside,
}

pub fn update_level(game: &mut GameState) -> Vec<DebugData> {
    update_camera(game);

    update_hole_physics(&mut game.objects.balls, &game.level.holes)
}

fn update_camera(game: &GameState) {
    set_camera(&Camera2D::from_display_rect(Rect::new(
        game.camera.x,
        game.camera.y,
        config::SCREEN_W,
        config::SCREEN_H,
    )));
}

fn update_hole_physics(balls: &mut Vec<Ball>, holes: &Vec<Hole>) -> Vec<DebugData> {
    let mut debug = vec![];
    for ball in balls.iter_mut() {
        if !ball.active {
            continue;
        }
        for hole in holes {
            // Determine nearest point on hole rim
            let edge: Vec2 = hole.pos + (ball.pos.truncate() - hole.pos).normalize() * hole.radius;

            // Find nearest point on wall
            let wall: Vec3;
            let ball_state = if (hole.pos.distance(ball.pos.truncate()) < hole.radius - BALL_RADIUS)
            {
                wall = edge.extend(f32::min(0.0, ball.pos.z));
                BallState::Inside
            } else if (hole.pos.distance(ball.pos.truncate()) < hole.radius) {
                wall = edge.extend(f32::min(0.0, ball.pos.z));
                BallState::InsideEdge
            } else if (hole.pos.distance(ball.pos.truncate()) < hole.radius + BALL_RADIUS) {
                wall = ball.pos.truncate().extend(0.0);
                BallState::OutsideEdge
            } else {
                wall = ball.pos.truncate().extend(0.0);
                BallState::Outside
            };

            debug.push(DebugData::circle(wall, 0.01, GREEN));
            debug.push(DebugData::line(
                Vec3::new(hole.pos.x, 0.0, 0.0),
                Vec3::new(hole.pos.x, hole.pos.y - hole.radius, 0.0),
                WHITE,
            ));
            debug.push(DebugData::line(
                Vec3::new(hole.pos.x, hole.pos.y + hole.radius, 0.0),
                Vec3::new(hole.pos.x, 10.0, 0.0),
                WHITE,
            ));
            debug.push(DebugData::line(
                Vec3::new(hole.pos.x, hole.pos.y - hole.radius, 0.0),
                Vec3::new(hole.pos.x, hole.pos.y - hole.radius, -5.0),
                WHITE,
            ));
            debug.push(DebugData::line(
                Vec3::new(hole.pos.x, hole.pos.y + hole.radius, 0.0),
                Vec3::new(hole.pos.x, hole.pos.y + hole.radius, -5.0),
                WHITE,
            ));

            // Determine normal vectors
            let unit_normal = (hole.pos - edge).extend(0.0).normalize();
            let wall_normal = (ball.pos - wall).normalize();

            // Solve collision by moving ball in normal direction
            let distance: Vec3 = ball.pos - wall;
            let sgn = unit_normal.dot(distance).signum();
            let intrusion: f32 = ball.pos.distance(wall) - BALL_RADIUS;

            if (wall.distance(ball.pos) < BALL_RADIUS) && sgn > 0.0 {
                ball.pos -= wall_normal * intrusion * sgn;
            }

            // Stop at back
            if ball.pos.z < -3.0 * BALL_RADIUS {
                ball.pos.z = -3.0 * BALL_RADIUS;
                ball.vel.z = 0.0;
            }

            match ball_state {
                BallState::InsideEdge => {
                    debug.push(DebugData::text("ball_state", "inside-edge".to_string()));
                    let projection = Vec3::new(-wall_normal.y, wall_normal.x, unit_normal.z);
                    if (wall.distance(ball.pos) < BALL_RADIUS) && sgn > 0.0 {
                        // zero veocity along normal
                        let v_target = ball.vel - wall_normal.dot(ball.vel) * wall_normal;
                        let impulse = v_target - ball.vel;
                        ball.impulses.push(impulse);
                    }

                    debug.push(DebugData::line(
                        ball.pos,
                        ball.pos + projection * 0.2,
                        YELLOW,
                    ));
                }
                BallState::OutsideEdge => {
                    debug.push(DebugData::text("ball_state", "outside-edge".to_string()));
                }
                BallState::Inside => {
                    debug.push(DebugData::text("ball_state", "inside".to_string()));
                }
                BallState::Outside => {
                    debug.push(DebugData::text("ball_state", "outside".to_string()));
                }
            }
            debug.push(DebugData::line(wall, wall + wall_normal * 0.2, MAGENTA));
        }
    }
    debug
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
