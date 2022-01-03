use crate::Result;
use macroquad::prelude::*;
use rust_embed::RustEmbed;
use std::collections::HashMap;

#[derive(RustEmbed)]
#[folder = "assets/"]
pub struct Asset;

pub struct Resources {
    pub backgrounds: HashMap<String, Texture2D>,
    pub splash: Texture2D,
    pub font_menu: Font,
    pub font_score: Font,
}

impl Resources {
    pub async fn new() -> Result<Self> {
        let splash_data = Asset::get("splash_example.png").ok_or("Could not load splash")?;
        let splash = Texture2D::from_file_with_format(&splash_data.data, None);
        let font_menu_data = Asset::get("Helltown-eg8p.ttf").ok_or("Could not load font")?;
        let font_menu = load_ttf_font_from_bytes(&font_menu_data.data)?;

        let font_score_data =
            Asset::get("PinballChallengeDeluxe-ae6g.ttf").ok_or("Could not load font")?;
        let font_score = load_ttf_font_from_bytes(&font_score_data.data)?;

        let backgrounds = Self::load_backgrounds()?;
        Ok(Resources {
            backgrounds,
            splash,
            font_menu,
            font_score,
        })
    }

    fn load_backgrounds() -> Result<HashMap<String, Texture2D>> {
        let mut backgrounds: HashMap<String, Texture2D> = HashMap::new();
        let file_names =
            Asset::iter().filter(|name| name.starts_with("level_") && name.ends_with(".png"));
        for name in file_names {
            let data = Asset::get(name.as_ref())
                .ok_or(format!("Could not load background \"{}\"", name))?
                .data;
            backgrounds.insert(
                name.to_string(),
                Texture2D::from_file_with_format(&data, None),
            );
        }
        Ok(backgrounds)
    }
}
