use macroquad::file::FileError;
use macroquad::prelude::*;
pub struct Resources {
    pub background: Texture2D,
    pub ball: Texture2D,
    pub font: Font,
}

impl Resources {
    pub async fn new() -> Result<Self, FileError> {
        let background = load_texture("assets/level_1_bg_small_tex.png").await?;
        let ball = load_texture("assets/ball.png").await?;
        let font = load_ttf_font("assets/Helltown-eg8p.ttf").await.unwrap();
        Ok(Resources {
            background,
            ball,
            font,
        })
    }
}
