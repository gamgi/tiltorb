use crate::{
    config::SCALE,
    debug::DebugData,
    game::balls::BALL_RADIUS,
    input::Input,
    resources::Resources,
    state::{Actuator, Ball},
};
use macroquad::{
    experimental::collections::storage,
    input::{is_mouse_button_down, mouse_position, MouseButton},
    math::{Vec2, Vec3},
    prelude::*,
};

const ACTUATOR_VEL: f32 = 2.0; // m/s TODO does not match reality
const ACTUATOR_STIFFNESS: f32 = 0.9;
const ACTUATOR_DAMPING: f32 = 8.0;
pub const ACTUATOR_WIDTH: f32 = 100. / SCALE; // px / scale
pub const ACTUATOR_Z: f32 = BALL_RADIUS * 1.2;
pub const ROD_RADIUS: f32 = 0.008;

pub fn update_actuators(actuators: &mut [Actuator; 2], input: &Input, dt: f32) {
    let actuator_y_mean = (actuators[0].pos.y + actuators[1].pos.y) / 2.0;
    for (actuator, actuator_input) in actuators.iter_mut().zip(input.actuators) {
        let target_vel = actuator_input * ACTUATOR_VEL;
        let delta_vel = target_vel - actuator.vel;

        // Actuator velocity follows Hooke's law F = -k*x - b*v
        // where k and b are stiffness and damping constants
        actuator.vel += dt * (-ACTUATOR_STIFFNESS * delta_vel - actuator.vel * ACTUATOR_DAMPING);
        actuator.pos.y += dt * actuator.vel;
        // Clamp distance
        actuator.pos.y = actuator
            .pos
            .y
            .clamp(actuator_y_mean - 0.2, actuator_y_mean + 0.2);
    }
}

pub fn update_rod_physics(balls: &mut Vec<Ball>, actuators: &[Actuator; 2]) {
    let mut debug = storage::get_mut::<Vec<DebugData>>();
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
        let actuator = actuators[0].pos.extend(ACTUATOR_Z);
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
}

pub fn draw_rod(actuators: &[Actuator; 2], rod_angle: f32) {
    let resources = storage::get_mut::<Resources>();
    // Actuators

    // Color  for actuator indicators
    let p = interp_cubic_ease((rod_angle / std::f32::consts::PI * 180.).abs() / 2.);
    let color = Color::from_rgba(10, 10 + (80. * (1. - p)) as u8, 10, 255);
    for ref actuator in actuators.iter() {
        draw_rectangle(
            (actuator.pos.x - ACTUATOR_WIDTH / 2.) * SCALE + 5.,
            actuator.pos.y * SCALE + 5.,
            90.0,
            140.0,
            color,
        );
        draw_texture(
            resources.actuator_fg,
            (actuator.pos.x - ACTUATOR_WIDTH / 2.) * SCALE,
            actuator.pos.y * SCALE,
            WHITE,
        );
    }

    // Rod
    let rod_pos = (actuators[0].pos + actuators[1].pos) / 2.0;
    let rod_length = SCALE * 1.5;
    draw_texture_ex(
        resources.rod,
        rod_pos.x * SCALE - rod_length / 2.,
        rod_pos.y * SCALE - ROD_RADIUS * SCALE,
        WHITE,
        DrawTextureParams {
            dest_size: Some(Vec2::new(rod_length, ROD_RADIUS * 2. * SCALE)),
            rotation: rod_angle,
            ..Default::default()
        },
    );
}

fn seesaw_unit_vec(actuators: &[Actuator; 2]) -> Vec3 {
    let delta_y = actuators[1].pos.y - actuators[0].pos.y;
    let delta_x = actuators[1].pos.x - actuators[0].pos.x;
    Vec3::new(delta_x, delta_y, 0.0).normalize()
}

pub fn rod_angle(actuators: &[Actuator; 2]) -> f32 {
    let delta = actuators[1].pos - actuators[0].pos;
    f32::atan2(delta.y, delta.x)
}

fn interp_cubic_ease(p: f32) -> f32 {
    match p {
        p if p <= 0. => 0.,
        p if p >= 1. => 1.,
        p if p < 0.5 => 4. * f32::powf(p, 3.),
        p => 1. - f32::powf(-2. * p + 2., 3.) / 2.,
    }
}
