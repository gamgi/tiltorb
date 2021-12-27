use crate::{
    config,
    config::SCALE,
    debug::DebugData,
    game::game::{BALL_RADIUS, ROD_RADIUS},
    input::Input,
    state::{Actuator, Ball, GameState},
};
use macroquad::{
    input::{is_mouse_button_down, mouse_position, MouseButton},
    math::{Vec2, Vec3},
    prelude::*,
    time::get_frame_time,
};

const ACTUATOR_VEL: f32 = 2.0; // m/s TODO does not match reality
const ACTUATOR_STIFFNESS: f32 = 0.9;
const ACTUATOR_DAMPING: f32 = 8.0;
const _BALL_MASS: f32 = 0.15; // kg

pub fn update_actuators(actuators: &mut [Actuator; 2], input: &Input) {
    let dt = get_frame_time();
    for (actuator, actuator_input) in actuators.iter_mut().zip(input.actuators) {
        let target_vel = actuator_input * ACTUATOR_VEL;
        let delta_vel = target_vel - actuator.vel;

        // Actuator velocity follows Hooke's law F = -k*x - b*v
        // where k and b are stiffness and damping constants
        actuator.vel += dt * (-ACTUATOR_STIFFNESS * delta_vel - actuator.vel * ACTUATOR_DAMPING);
        actuator.pos.y += dt * actuator.vel;
    }
}

pub fn update_rod_physics(balls: &mut Vec<Ball>, actuators: &[Actuator; 2]) -> Vec<DebugData> {
    let mut debug = vec![];
    for ball in balls.iter_mut() {
        if !ball.active {
            continue;
        }
        if is_mouse_button_down(MouseButton::Left) {
            ball.pos = (Vec2::from(mouse_position()) / SCALE * 2.0).extend(BALL_RADIUS);
            ball.vel = Vec3::new(0.0, 0.0, 0.0);
        }
        debug.push(DebugData::circle(ball.pos, 0.03, BLUE));

        // Determine nearest point on rod
        let actuator = actuators[0].pos.extend(BALL_RADIUS * 1.01);
        let rod = actuator
            + seesaw_unit_vec(actuators) * (ball.pos - actuator).dot(seesaw_unit_vec(actuators));
        // Determine normal vectors
        let rod_normal = (ball.pos - rod).normalize();

        // Solve collision by moving ball in normal direction
        let distance: Vec3 = ball.pos - rod;
        let sgn = Vec3::new(0.0, -1.0, 0.0).dot(distance).signum();
        let intrusion: f32 = ball.pos.distance(rod) - (BALL_RADIUS + ROD_RADIUS);
        if (rod.distance(ball.pos) < (BALL_RADIUS + ROD_RADIUS)) && sgn > 0.0 {
            ball.pos -= rod_normal * intrusion * sgn;
        } else {
            continue;
        }

        // Correct velocity
        ball.impulses.push(-rod_normal.dot(ball.vel) * rod_normal);
        // Debug
        debug.push(DebugData::text("rod hit", "hit".to_string()));
        debug.push(DebugData::line(rod, rod + rod_normal * 0.2, RED));
        debug.push(DebugData::circle(rod, 0.01, RED));
    }
    debug
}

pub fn update_balls(balls: &mut Vec<Ball>) {
    let dt = get_frame_time();
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

/// Calculate seesaw y coordinate at x
///
/// Graphically:
/// actuator[0](x,y) --- (at_x, ?) --- actuator[1](x,y)
fn _seesaw_y(at_x: f32, actuators: &[Actuator; 2]) -> f32 {
    // TODO vectors
    let delta_y = actuators[1].pos.y - actuators[0].pos.y;
    let relative_x = at_x - actuators[0].pos.x;
    let delta_x = actuators[1].pos.x - actuators[0].pos.x;
    let fraction_x = relative_x / delta_x;

    actuators[0].pos.y + delta_y * fraction_x
}

fn seesaw_unit_vec(actuators: &[Actuator; 2]) -> Vec3 {
    let delta_y = actuators[1].pos.y - actuators[0].pos.y;
    let delta_x = actuators[1].pos.x - actuators[0].pos.x;
    Vec3::new(delta_x, delta_y, 0.0).normalize()
}
