use crate::{
    input::Input,
    state::{Event, MenuState},
};
use macroquad::{prelude::*, text::measure_text, time::get_time};

pub fn update_menu(menu: &mut MenuState, input: &Input) -> Option<Event> {
    if input.menu_up {
        menu.selected = (menu.selected as i32 - 1).rem_euclid(menu.options.len() as i32) as usize;
    } else if input.menu_down {
        menu.selected = (menu.selected + 1).rem_euclid(menu.options.len());
    }

    if input.enter {
        return Some(Event::Selected(
            menu.options
                .get(menu.selected)
                .unwrap()
                .to_string()
                .to_lowercase(),
        ));
    }
    None
}

pub fn draw_menu(menu: &MenuState) {
    let scale = f32::sin(10.0 * get_time() as f32) * 2.5;
    for (i, text) in menu.options.iter().enumerate() {
        if menu.selected == i {
            draw_centered_text(text, 250.0 + 30.0 + i as f32 * 30.0, 32.0 + scale);
        } else {
            draw_centered_text(text, 250.0 + 30.0 + i as f32 * 30.0, 30.0);
        }
    }
}

fn draw_centered_text(text: &str, y: f32, font_size: f32) {
    let text_size = measure_text(text, None, font_size as u16, 1.0);
    draw_text(
        text,
        screen_width() / 2.0 - text_size.width / 2.0,
        y,
        font_size,
        WHITE,
    );
}
