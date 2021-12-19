use crate::input::Input;
use crate::physics;
use crate::state::GameState;
use macroquad::prelude::*;

// TODO update_state
pub fn update_game(game: &mut GameState, input: &Input) {
    // Actuators
    physics::update_actuators(&mut game.objects.actuators, input);
    // Balls
    physics::update_balls(&mut game.objects.balls, &game.objects.actuators);
    // state
}

pub fn draw_game(game: &GameState) {
    // Balls
    for ref ball in game.objects.balls.iter() {
        draw_circle(ball.pos.x, ball.pos.y, 15.0, BLUE);
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
