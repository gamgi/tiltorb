use macroquad::text::{draw_text_ex, measure_text, TextParams};
macro_rules! return_ok_if_some {
    ( $e:expr ) => {
        match $e {
            Some(v) => {
                return Ok(Some(v));
            }
            None => {}
        }
    };
}

macro_rules! return_if_some {
    ( $e:expr ) => {
        if let Some(v) = $e {
            return Some(v);
        }
    };
}
pub(crate) use return_if_some;
pub(crate) use return_ok_if_some;

pub fn draw_centered_text(text: &str, x: f32, y: f32, text_params: TextParams) {
    let text_size = measure_text(text, None, text_params.font_size as u16, 1.0);
    draw_text_ex(text, x - text_size.width / 2.0, y, text_params);
}
