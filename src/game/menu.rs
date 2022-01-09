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

const FONT_SIZE_TITLE: f32 = 128.;
const FONT_SIZE_MENU: f32 = 96.;

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
    let y_start = SCREEN_H / 2. - 80.;
    // Title
    draw_centered_text(
        env!("CARGO_CRATE_NAME"),
        SCREEN_W / 2. - 10.,
        y_start - FONT_SIZE_MENU * 2. - 10.,
        get_text_params(
            &resources,
            FONT_SIZE_TITLE,
            Color::from_rgba(254, 93, 29, 255),
        ),
    );
    draw_centered_text(
        env!("CARGO_CRATE_NAME"),
        SCREEN_W / 2.,
        y_start - FONT_SIZE_MENU * 2.,
        get_text_params(&resources, FONT_SIZE_TITLE, WHITE),
    );
    // Menu options
    let scale = f32::sin(10.0 * get_time() as f32) * 2.5;
    let y_start = SCREEN_H / 2. - 80.;
    for (i, text) in menu.options.iter().enumerate() {
        let font_size = FONT_SIZE_MENU + scale * ((menu.selected == i) as i32 as f32);
        let text_params = get_text_params(&resources, font_size, WHITE);
        if menu.selected == i {
            draw_centered_text(
                text,
                SCREEN_W / 2.,
                y_start + FONT_SIZE_MENU + i as f32 * FONT_SIZE_MENU,
                text_params,
            );
        } else {
            draw_centered_text(
                text,
                SCREEN_W / 2.,
                y_start + FONT_SIZE_MENU + i as f32 * FONT_SIZE_MENU,
                text_params,
            );
        }
    }
}

fn get_text_params(resources: &Resources, font_size: f32, color: Color) -> TextParams {
    TextParams {
        font_size: font_size as u16,
        font: resources.font_menu,
        color,
        ..Default::default()
    }
}
