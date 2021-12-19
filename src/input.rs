use macroquad::prelude::*;
#[derive(Debug, PartialEq)]
pub struct Input {
    pub actuator_left: f32,
    pub actuator_right: f32,
    pub menu_up: bool,
    pub menu_down: bool,
    pub enter: bool,
}

fn read_updown(up: KeyCode, down: KeyCode) -> f32 {
    is_key_down(up) as i32 as f32 - is_key_down(down) as i32 as f32
}

pub fn update_input() -> Input {
    Input {
        actuator_left: read_updown(KeyCode::W, KeyCode::S),
        actuator_right: read_updown(KeyCode::Up, KeyCode::Down),
        menu_up: is_key_pressed(KeyCode::Up),
        menu_down: is_key_pressed(KeyCode::Down),
        enter: is_key_pressed(KeyCode::Enter),
    }
}
