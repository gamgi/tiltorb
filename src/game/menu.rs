use crate::{
    input::Input,
    resources::Resources,
    state::{Event, MenuState},
};
use macroquad::{
    experimental::collections::storage, prelude::*, text::measure_text, time::get_time,
};

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
    let resources = storage::get_mut::<Resources>();
    let scale = f32::sin(10.0 * get_time() as f32) * 2.5;
    for (i, text) in menu.options.iter().enumerate() {
        let font_size = 48.0 + scale * ((menu.selected == i) as i32 as f32);
        let text_params = get_text_params(&resources, font_size);
        if menu.selected == i {
            draw_centered_text(text, 250.0 + 48.0 + i as f32 * 48.0, text_params);
        } else {
            draw_centered_text(text, 250.0 + 48.0 + i as f32 * 48.0, text_params);
        }
    }
}
fn get_text_params(resources: &Resources, font_size: f32) -> TextParams {
    TextParams {
        font_size: font_size as u16,
        font: resources.font,
        ..Default::default()
    }
}
fn draw_centered_text(text: &str, y: f32, text_params: TextParams) {
    let text_size = measure_text(text, None, text_params.font_size as u16, 1.0);
    draw_text_ex(
        text,
        screen_width() / 2.0 - text_size.width / 2.0,
        y,
        text_params,
    );
}
