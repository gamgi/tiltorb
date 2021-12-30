use macroquad::miniquad::conf::Conf as Window;
use macroquad::prelude::*;

pub const SCALE: f32 = 1080.0; // 1m in pixels
pub const WINDOW_W: i32 = 540;
pub const WINDOW_H: i32 = 960;
pub const SCREEN_W: f32 = 1080.0;
pub const SCREEN_H: f32 = 1920.0;
const GAME_NAME: &str = "Arcade Game";

pub fn window_conf() -> Window {
    Window {
        window_title: GAME_NAME.to_owned(),
        window_width: WINDOW_W,
        window_height: WINDOW_H,
        ..Default::default()
    }
}
