use crate::{
    config,
    config::SCALE,
    input::Input,
    state::{Actuator, Ball},
};
use macroquad::{math::Vec2, time::get_frame_time};

const GRAVITY: (f32, f32) = (0.0, 9.81); // m/s
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

pub fn update_balls(balls: &mut Vec<Ball>, actuators: &[Actuator; 2]) {
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
            // reflect ball velocity on x-axis
            ball.vel.x *= -WALL_DAMPING;
        }
        if ball.pos.y >= max_y {
            // reset ball y
            ball.pos.y = max_y;
            // project ball velocity on seesaw
            ball.vel = seesaw_unit_vec(actuators).dot(ball.vel) * seesaw_unit_vec(actuators);
        }
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

fn seesaw_unit_vec(actuators: &[Actuator; 2]) -> Vec2 {
    let delta_y = actuators[1].pos.y - actuators[0].pos.y;
    let delta_x = actuators[1].pos.x - actuators[0].pos.x;
    Vec2::new(delta_x, delta_y).normalize()
}
