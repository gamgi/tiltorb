use macroquad::prelude::*;
const DURATION: f64 = 0.33;

pub fn draw_transition(elapsed: f64, out: bool) {
    let y = ((-1. + out as i32 as f32) + elapsed as f32) * screen_height();
    draw_rectangle(
        0.0,
        y * 2.,
        screen_width() * 2.,
        screen_height() * 2.,
        BLACK,
    );
}

pub fn elapsed(start_time: f64) -> f64 {
    (get_time() - start_time) / DURATION
}
