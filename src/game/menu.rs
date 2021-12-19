use crate::input::Input;
use crate::state::MenuState;
use macroquad::prelude::*;

pub fn update_menu(menu: &mut MenuState, input: &Input) {
    if input.menu_up {
        menu.selected = (menu.selected as i32 - 1).rem_euclid(menu.options.len() as i32) as usize;
    } else if input.menu_down {
        menu.selected = (menu.selected + 1).rem_euclid(menu.options.len());
    }
}
