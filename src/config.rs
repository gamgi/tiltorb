use macroquad::miniquad::conf::Conf as Window;
use macroquad::prelude::*;

pub const SCREEN_W: i32 = 960;
pub const SCREEN_H: i32 = 540;
pub const WINDOW_W: f32 = 1920.0;
pub const WINDOW_H: f32 = 1080.0;
const GAME_NAME: &str = "Tilt Orb";

pub fn window_conf() -> Window {
    Window {
        window_title: GAME_NAME.to_owned(),
        window_width: SCREEN_W,
        window_height: SCREEN_H,
        ..Default::default()
    }
}

pub fn camera_conf() -> Camera2D {
    Camera2D::from_display_rect(Rect::new(0.0, 0.0, WINDOW_W, WINDOW_H))
}
