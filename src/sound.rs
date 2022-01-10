use crate::{resources::SoundResources, state::Event};
use macroquad::{audio::play_sound_once, experimental::collections::storage};

pub fn play_event_sound(event: &Option<Event>) {
    let resources = storage::get_mut::<SoundResources>();
    if let Some(ev) = event {
        match ev {
            Event::GameCompleted => play_sound_once(
                *resources
                    .sounds
                    .get("sound_example.wav")
                    .expect("Resource for sound_game_completed not found"),
            ),
            Event::GameEnded => play_sound_once(
                *resources
                    .sounds
                    .get("sound_example.wav")
                    .expect("Resource for sound_game_completed not found"),
            ),
            Event::RoundLost => play_sound_once(
                *resources
                    .sounds
                    .get("sound_example.wav")
                    .expect("Resource for sound_lost not found"),
            ),
            Event::RoundCompleted => play_sound_once(
                *resources
                    .sounds
                    .get("sound_example.wav")
                    .expect("Resource for sound_pickup not found"),
            ),
            Event::MenuSelected(_) => play_sound_once(
                *resources
                    .sounds
                    .get("sound_example.wav")
                    .expect("Resource for sound_select not found"),
            ),
            _ => {}
        };
    }
}
