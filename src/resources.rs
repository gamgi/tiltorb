use crate::Result;
use macroquad::{
    audio::{load_sound_from_bytes, Sound},
    prelude::*,
};
use rust_embed::RustEmbed;
use std::collections::HashMap;

#[derive(RustEmbed)]
#[folder = "assets/"]
pub struct Asset;

pub struct Resources {
    pub actuator_fg: Texture2D,
    pub actuator_bg: Texture2D,
    pub backgrounds: HashMap<String, Texture2D>,
    pub font_menu: Font,
    pub font_score: Font,
    pub ball_fg: Texture2D,
    pub ball_bg: Texture2D,
    pub key: Texture2D,
    pub rod: Texture2D,
    pub splash: Texture2D,
    pub sounds: HashMap<String, Sound>,
}

impl Resources {
    pub async fn new() -> Result<Self> {
        let actuator_fg_data = Asset::get("actuator_fg.png").ok_or("Could not load actuator")?;
        let actuator_fg = Texture2D::from_file_with_format(&actuator_fg_data.data, None);
        let actuator_bg_data = Asset::get("actuator_bg.png").ok_or("Could not load actuator")?;
        let actuator_bg = Texture2D::from_file_with_format(&actuator_bg_data.data, None);

        let backgrounds = Self::load_backgrounds()?;
        let ball_fg_data = Asset::get("ball_fg.png").ok_or("Could not load ball")?;
        let ball_fg = Texture2D::from_file_with_format(&ball_fg_data.data, None);
        let ball_bg_data = Asset::get("ball_bg.png").ok_or("Could not load ball")?;
        let ball_bg = Texture2D::from_file_with_format(&ball_bg_data.data, None);

        let font_menu_data = Asset::get("Helltown-eg8p.ttf").ok_or("Could not load font")?;
        let font_menu = load_ttf_font_from_bytes(&font_menu_data.data)?;

        let font_score_data =
            Asset::get("PinballChallengeDeluxe-ae6g.ttf").ok_or("Could not load font")?;
        let font_score = load_ttf_font_from_bytes(&font_score_data.data)?;

        let key_data = Asset::get("key.png").ok_or("Could not load key")?;
        let key = Texture2D::from_file_with_format(&key_data.data, None);

        let rod_data = Asset::get("rod.png").ok_or("Could not load rod")?;
        let rod = Texture2D::from_file_with_format(&rod_data.data, None);

        let splash_data = Asset::get("splash_example.png").ok_or("Could not load splash")?;
        let splash = Texture2D::from_file_with_format(&splash_data.data, None);

        let sounds = Self::load_sounds().await?;
        Ok(Resources {
            actuator_fg,
            actuator_bg,
            backgrounds,
            ball_fg,
            ball_bg,
            font_menu,
            font_score,
            key,
            rod,
            splash,
            sounds,
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

    async fn load_sounds() -> Result<HashMap<String, Sound>> {
        let mut sounds: HashMap<String, Sound> = HashMap::new();
        let file_names =
            Asset::iter().filter(|name| name.starts_with("sound_") && name.ends_with(".wav"));
        for name in file_names {
            let data = Asset::get(name.as_ref())
                .ok_or(format!("Could not load background \"{}\"", name))?
                .data;
            sounds.insert(name.to_string(), load_sound_from_bytes(&data).await?);
        }
        Ok(sounds)
    }
}
