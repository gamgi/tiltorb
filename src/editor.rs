use crate::{
    config::SCALE,
    game::{level, rod},
    state::{EditorState, Event, GameLevelState, GameState, Hole},
    Result,
};
use macroquad::prelude::*;
use std::fs::File;

pub fn update_editor(game: &mut GameState, editor: &mut EditorState) -> Option<Event> {
    editor.radius = ((editor.radius + mouse_wheel().1 * 6.).clamp(8., 100.) / 2.).round() * 2.;
    if is_mouse_button_pressed(MouseButton::Left) {
        game.level.holes.push(Hole {
            pos: Vec2::from(mouse_position()) / SCALE * 2.,
            radius: editor.radius / 500.,
        })
    } else if is_key_pressed(KeyCode::Enter) {
        save(&game.level, "level_new.json").expect("Failed to save");
        return Some(Event::EditorClosed);
    } else if is_key_down(KeyCode::LeftSuper) && is_key_pressed(KeyCode::Z) {
        game.level.holes.pop();
    }

    None
}

pub fn draw_editor(game: &GameState, editor: &EditorState) {
    level::draw_background(game);
    level::draw_holes(game);
    rod::draw_rod(&game.objects.actuators, 0.);
    draw_rectangle(0., 0., screen_width() * 2., 120., BLACK);
    let mouse = Vec2::from(mouse_position()) / SCALE * 2.;
    draw_text(
        &format!("Editor - radius {}", editor.radius),
        30.0,
        30.0,
        48.0,
        YELLOW,
    );

    draw_circle(
        mouse.x * SCALE,
        mouse.y * SCALE,
        editor.radius / 500. * SCALE,
        YELLOW,
    );
}

fn save(level: &GameLevelState, file_name: &str) -> Result<()> {
    let file = File::create(file_name)?;
    serde_json::to_writer(&file, level)?;
    Ok(())
}
