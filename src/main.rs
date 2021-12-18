use macroquad::prelude::*;
mod config;
mod level;
use crate::config as conf;
use crate::{config::window_conf, level::draw_level};

#[macroquad::main(window_conf)]
async fn main() {
    let camera = conf::camera_conf();
    set_camera(&camera);
    loop {
        draw_level();
        next_frame().await
    }
}
