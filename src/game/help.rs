use crate::{
    config::{SCALE, SCREEN_H, SCREEN_W},
    input::Input,
    resources::Resources,
    state::GameState,
};
use macroquad::{experimental::collections::storage, prelude::*};

pub fn update_help(game: &mut GameState, input: &Input) {
    if !game.progress.show_help {
        return;
    }
    if input.actuators.iter().any(|a| *a > 0. || *a < 0.) {
        game.progress.show_help = false;
    }
}
pub fn draw_help(game: &GameState) {
    let resources = storage::get::<Resources>();
    let blinking = (get_time() * 5.).round().rem_euclid(2.) as u32 == 0;
    if blinking && game.progress.show_help {
        draw_key(
            &resources,
            (game.objects.actuators[0].pos.x) * SCALE - 30.,
            game.objects.actuators[0].pos.y * SCALE - 110.,
            "w",
        );
        draw_key(
            &resources,
            (game.objects.actuators[0].pos.x) * SCALE - 30.,
            game.objects.actuators[0].pos.y * SCALE,
            "s",
        );
        draw_key(
            &resources,
            (game.objects.actuators[1].pos.x) * SCALE - 70.,
            game.objects.actuators[1].pos.y * SCALE - 110.,
            "^",
        );
        draw_key(
            &resources,
            (game.objects.actuators[1].pos.x) * SCALE - 70.,
            game.objects.actuators[1].pos.y * SCALE,
            "v",
        );
    }
}

fn draw_key(resources: &Resources, x: f32, y: f32, character: &str) {
    draw_texture(resources.key, x, y, WHITE);
    draw_text(character, x + 34., y + 50., 64., BLACK);
}
