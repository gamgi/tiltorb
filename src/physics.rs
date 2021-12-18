use crate::input::Input;
use crate::state::{Actuator, Ball};
use macroquad::math::Vec2;

const GRAVITY: (f32, f32) = (0.0, 9.81);
const ACTUATOR_VEL: (f32, f32) = (4.0, 4.0);
const BALL_MASS: f32 = 10.0;

pub fn update_actuators(actuators: &mut [Actuator; 2], input: &Input) {
    actuators[0].pos.y -= input.actuator_left * ACTUATOR_VEL.0;
    actuators[1].pos.y -= input.actuator_right * ACTUATOR_VEL.1;
}

pub fn update_balls(balls: &mut Vec<Ball>, actuators: &[Actuator; 2]) {
    for ball in balls.iter_mut() {
        ball.pos += ball.vel;
        ball.vel += Vec2::from(GRAVITY) / BALL_MASS;
        let max_y = seesaw_y(ball.pos.x, actuators);

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
