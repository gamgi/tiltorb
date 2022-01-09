use crate::{
    config::{SCREEN_H, SCREEN_W},
    input::Input,
    resources::Resources,
    state::{Event, MenuState},
    utils::draw_centered_text,
};
use macroquad::{
    audio::play_sound_once, experimental::collections::storage, prelude::*, time::get_time,
};

const FONT_SIZE: f32 = 96.;

pub fn update_menu(menu: &mut MenuState, input: &Input) -> Option<Event> {
    let resources = storage::get_mut::<Resources>();
    if input.menu_up {
        play_sound_once(
            *resources
                .sounds
                .get("sound_example.wav")
                .expect("Resource for sound_click not found"),
        );
        menu.selected = (menu.selected as i32 - 1).rem_euclid(menu.options.len() as i32) as usize;
    } else if input.menu_down {
        play_sound_once(
            *resources
                .sounds
                .get("sound_example.wav")
                .expect("Resource for sound_click not found"),
        );
        menu.selected = (menu.selected + 1).rem_euclid(menu.options.len());
    }

    if input.enter {
        return Some(Event::MenuSelected(
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
    let y_start = SCREEN_H / 2. - 80.;
    for (i, text) in menu.options.iter().enumerate() {
        let font_size = FONT_SIZE + scale * ((menu.selected == i) as i32 as f32);
        let text_params = get_text_params(&resources, font_size);
        if menu.selected == i {
            draw_centered_text(
                text,
                SCREEN_W / 2.,
                y_start + FONT_SIZE + i as f32 * FONT_SIZE,
                text_params,
            );
        } else {
            draw_centered_text(
                text,
                SCREEN_W / 2.,
                y_start + FONT_SIZE + i as f32 * FONT_SIZE,
                text_params,
            );
        }
    }
}

fn get_text_params(resources: &Resources, font_size: f32) -> TextParams {
    TextParams {
        font_size: font_size as u16,
        font: resources.font_menu,
        ..Default::default()
    }
}
