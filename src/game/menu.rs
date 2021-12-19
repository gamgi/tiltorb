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

pub fn draw_menu(menu: &MenuState) {
    draw_text(
        &format!("menu {}", menu.selected.to_owned()),
        30.0,
        250.0,
        30.0,
        WHITE,
    );
    draw_text("menu", 30.0, 200.0, 30.0, WHITE);
}
