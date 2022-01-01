use crate::Result;
use macroquad::prelude::*;
use rust_embed::RustEmbed;
use std::collections::HashMap;

#[derive(RustEmbed)]
#[folder = "assets/"]
struct Asset;

pub struct Resources {
    pub backgrounds: HashMap<String, Texture2D>,
    pub font: Font,
}

impl Resources {
    pub async fn new() -> Result<Self> {
        let font_data = Asset::get("Helltown-eg8p.ttf").ok_or("Could not load font")?;
        let font = load_ttf_font_from_bytes(&font_data.data)?;

        let backgrounds = Self::load_backgrounds()?;
        Ok(Resources { backgrounds, font })
    }

    fn load_backgrounds() -> Result<HashMap<String, Texture2D>> {
        let mut backgrounds: HashMap<String, Texture2D> = HashMap::new();
        let file_names =
            Asset::iter().filter(|name| name.starts_with("level_") && name.ends_with(".png"));
        for name in file_names {
            let data = Asset::get(name.as_ref())
                .ok_or(format!("Could not load \"{}\"", name))?
                .data;
            backgrounds.insert(
                name.to_string(),
                Texture2D::from_file_with_format(&data, None),
            );
        }
        Ok(backgrounds)
    }
}
