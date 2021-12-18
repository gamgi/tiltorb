use crate::input::Input;
use crate::state::{Actuator, Ball};

pub fn update_actuators(actuators: &mut [Actuator; 2], input: &Input) {
    actuators[0].pos.y -= input.actuator_left;
    actuators[1].pos.y -= input.actuator_right;
}
pub fn update_balls(balls: &mut Vec<Ball>, actuators: &[Actuator; 2]) {}
