use crate::input::Input;
use crate::state::State;
use macroquad::prelude::*;

pub fn update_objects(state: &mut State, input: &Input) {
    match state {
        State::Game(game) => {
            game.objects.actuators[0].pos.y -= input.actuator_left;
            game.objects.actuators[1].pos.y -= input.actuator_right;
        }
        _ => {}
    }
}

pub fn draw_objects(state: &State) {
    match state {
        State::Menu(ref game) | State::Game(ref game) => {
            // Balls
            for ref ball in game.objects.balls.iter() {
                draw_circle(ball.pos.x, ball.pos.y, 15.0, YELLOW);
            }
            // Actuators
            for ref actuator in game.objects.actuators.iter() {
                draw_rectangle(actuator.pos.x, actuator.pos.y, 60.0, 60.0, GREEN);
            }
            // Seesaw
            draw_line(
                game.objects.actuators[0].pos.x,
                game.objects.actuators[0].pos.y,
                game.objects.actuators[1].pos.x,
                game.objects.actuators[1].pos.y,
                15.0,
                BLUE,
            );
        }
    }
}
