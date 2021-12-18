use crate::input::Input;
use crate::state::State;
use macroquad::prelude::*;

pub fn update_objects(state: &State, input: &Input) {}

pub fn draw_objects(state: &State) {
    match state {
        State::Menu(ref game) | State::Game(ref game) => {
            for ref ball in game.objects.balls.iter() {
                draw_circle(ball.pos.x, ball.pos.y, 15.0, YELLOW);
            }
        }
    }
}
