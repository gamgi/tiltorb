use macroquad::file::FileError;
use macroquad::prelude::*;
pub struct Resources {
    pub background: Texture2D,
    pub ball: Texture2D,
}

impl Resources {
    pub async fn new() -> Result<Self, FileError> {
        let background = load_texture("assets/testbg_small.png").await?;
        let ball = load_texture("assets/ball.png").await?;
        Ok(Resources { background, ball })
    }
}
