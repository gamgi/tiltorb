use crate::{
    config::SCREEN_W,
    resources::Resources,
    state::{DisplayState, GameState},
};
use macroquad::{experimental::collections::storage, prelude::*, time::get_time};

const FONT_SIZE: u16 = 164;
const FONT_HEIGHT: f32 = 0.57 * FONT_SIZE as f32;
const MESSAGE_DURATION: f64 = 1.5; // sec

pub fn update_display(display: &mut DisplayState) {
    if display.message.is_some() && (get_time() - display.start_time) > MESSAGE_DURATION {
        display.message = None;
    }
}

pub fn draw_display(game: &GameState, display: &DisplayState) {
    let resources = storage::get_mut::<Resources>();
    let text: String = match &display.message {
        Some(message) => format!("{: ^9}", message),
        None => format!(
            "{:0>2} {: >6}",
            game.progress.time().to_string(),
            game.progress.score.to_string()
        ),
    };
    let text_params_front = TextParams {
        font_size: FONT_SIZE,
        font: resources.font_score,
        color: RED,
        ..Default::default()
    };
    let text_params_back = TextParams {
        color: Color::from_rgba(30, 30, 30, 255),
        ..text_params_front
    };
    draw_rectangle(0., 0., SCREEN_W * 2., FONT_HEIGHT * 2. - 12., BLACK);
    draw_text_ex("000000000", 28., 48. + FONT_HEIGHT, text_params_back);
    draw_text_ex("*********", 28., 48. + FONT_HEIGHT, text_params_back);

    let blinking = (get_time() * 5.).round().rem_euclid(2.) as u32 == 0;
    if blinking || display.message.is_none() {
        draw_text_ex(&text, 28., 48. + FONT_HEIGHT, text_params_front);
    }
}
