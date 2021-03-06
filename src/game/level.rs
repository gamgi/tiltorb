use crate::{
    config,
    config::SCALE,
    debug::DebugData,
    game::balls::BALL_RADIUS,
    resources::Resources,
    state::{Ball, Event, GameState, Hole},
};
use macroquad::experimental::collections::storage;
use macroquad::math::{Vec2, Vec3};
use macroquad::prelude::*;

const WALL_DAMPING: f32 = 0.4;
const _BALL_MASS: f32 = 0.15; // kg

pub fn update_level(game: &mut GameState) -> Option<Event> {
    update_edge_physics(&mut game.objects.balls);
    update_hole_physics(&mut game.objects.balls, &game.level.holes);
    update_state(game)
}

fn update_hole_physics(balls: &mut Vec<Ball>, holes: &Vec<Hole>) {
    let mut debug = storage::get_mut::<Vec<DebugData>>();
    for ball in balls.iter_mut() {
        if !ball.active {
            continue;
        }
        ball.in_hole = None;
        for (i, hole) in holes.iter().enumerate() {
            let is_last_hole = i == holes.len() - 1;
            // Determine nearest point on hole rim
            let edge: Vec2 = hole.pos + (ball.pos.truncate() - hole.pos).normalize() * hole.radius;

            // Find nearest point on wall
            let wall = if hole.pos.distance(ball.pos.truncate()) < hole.radius - BALL_RADIUS {
                ball.in_hole = Some(i);
                edge.extend(f32::min(0.0, ball.pos.z))
            } else if hole.pos.distance(ball.pos.truncate()) < hole.radius {
                ball.in_hole = Some(i);
                edge.extend(f32::min(0.0, ball.pos.z))
            } else if is_last_hole && ball.in_hole.is_none() {
                // use "background" as wall
                ball.pos.truncate().extend(0.0)
            } else {
                continue;
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
            } else {
                continue;
            }

            // Correct velocity
            let impulse = -wall_normal.dot(ball.vel) * wall_normal;
            ball.impulses.push(impulse);

            // Debug
            let projection = Vec3::new(-wall_normal.y, wall_normal.x, unit_normal.z);

            debug.push(DebugData::line(
                ball.pos,
                ball.pos + projection * 0.2,
                YELLOW,
            ));
            debug.push(DebugData::line(wall, wall + wall_normal * 0.2, MAGENTA));
        }

        debug.push(DebugData::text("in hole", format!("{:?}", ball.in_hole)));
    }
}

fn update_edge_physics(balls: &mut Vec<Ball>) {
    for ball in balls.iter_mut() {
        if !ball.active {
            continue;
        }
        // X-axis
        let min_x = BALL_RADIUS;
        let max_x = config::SCREEN_W / SCALE - BALL_RADIUS;
        if ball.pos.x < min_x || ball.pos.x > max_x {
            ball.pos.x = ball.pos.x.clamp(min_x, max_x);
            let impulse = Vec3::new(-ball.vel.x * (1.0 + WALL_DAMPING), 0.0, 0.0); // * mass
            ball.impulses.push(impulse);
        }
        // Z-axis
        let min_z = -3.0 * BALL_RADIUS;
        let max_z = BALL_RADIUS;
        if ball.pos.z < min_z || ball.pos.z > max_z {
            ball.pos.z = ball.pos.z.clamp(min_z, max_z);
            ball.impulses.push(Vec3::new(0.0, 0.0, -ball.vel.z));
        }
    }
}

fn update_state(game: &mut GameState) -> Option<Event> {
    for ball in game.objects.balls.iter_mut() {
        if !ball.active {
            continue;
        }
        if ball.pos.z < -2. * BALL_RADIUS {
            if let Some(current_hole) = ball.in_hole {
                ball.active = false;
                if game.get_goal_hole() == current_hole {
                    if game.progress.goal_index >= game.level.goals.len() - 1 {
                        return Some(Event::GameCompleted);
                    } else {
                        return Some(Event::RoundCompleted);
                    }
                } else {
                    if game.progress.balls_left < 1 {
                        return Some(Event::GameEnded);
                    } else {
                        game.progress.balls_left -= 1;
                        return Some(Event::RoundLost);
                    }
                }
            }
        }
    }
    None
}

pub fn draw_holes(game: &GameState) {
    for (i, hole) in game.level.holes.iter().enumerate() {
        let color = match game.get_goal_hole() == i {
            true => YELLOW,
            false => BLACK,
        };
        draw_circle(
            hole.pos.x * SCALE,
            hole.pos.y * SCALE,
            hole.radius * SCALE,
            color,
        );
    }
}

pub fn draw_background(game: &GameState) {
    let resources = storage::get_mut::<Resources>();
    draw_texture_ex(
        *resources
            .backgrounds
            .get(&game.level.background_image)
            .expect("Resource for background_image not found"),
        0.0,
        0.0,
        WHITE,
        DrawTextureParams {
            dest_size: Some(vec2(1080., 1920.)),
            ..Default::default()
        },
    );
}
