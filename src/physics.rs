use crate::{
    config,
    config::SCALE,
    game::game::BALL_RADIUS,
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
const WALL_DAMPING: f32 = 0.4;

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

pub fn update_rod_physics(balls: &mut Vec<Ball>, actuators: &[Actuator; 2]) {
    let dt = get_frame_time();
    for ball in balls.iter_mut() {
        if !ball.active {
            continue;
        }
        ball.vel += dt * Vec2::from(GRAVITY);
        ball.pos += dt * ball.vel;
        let max_y = seesaw_y(ball.pos.x, actuators);

        if ball.pos.x < 0.0 || ball.pos.x > (config::SCREEN_W / SCALE) {
            // reset ball x
            ball.pos.x = ball.pos.x.clamp(0.0, config::SCREEN_W / SCALE);

            let impulse = Vec3::new(-ball.vel.x * (1.0 + WALL_DAMPING), 0.0, 0.0); // * mass
            ball.impulses.push(impulse);
        }
        if ball.pos.y >= max_y {
            // reset ball y
            ball.pos.y = max_y;
            // project ball velocity on seesaw
            let v_target = seesaw_unit_vec(actuators).dot(ball.vel) * seesaw_unit_vec(actuators);
            let impulse = v_target - ball.vel; // * mass
            ball.impulses.push(impulse);
        }
    }
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
fn seesaw_y(at_x: f32, actuators: &[Actuator; 2]) -> f32 {
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
