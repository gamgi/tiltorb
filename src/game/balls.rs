use crate::{debug::DebugData, state::Ball};
use macroquad::{math::Vec3, time::get_frame_time};

pub fn update_balls(balls: &mut Vec<Ball>) -> Vec<DebugData> {
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
    Vec::new()
}
