use crate::config::{SCREEN_H, SCREEN_W};
use macroquad::math::Vec2;

#[derive(Debug, PartialEq)]
pub enum State {
    Initial,
    Loading,
    Menu(GameState, MenuState),
    Game(GameState),
}

#[derive(Debug, PartialEq)]
pub enum Event {
    Initialized,
    Loaded,
    Selected(String),
}

impl State {
    pub fn transition(self, event: Event) -> Self {
        match (&self, event) {
            (State::Initial, Event::Initialized) => State::Loading,
            (State::Loading, Event::Loaded) => State::Menu(
                GameState::new(),
                MenuState {
                    selected: 0,
                    options: vec!["Start".to_string(), "Quit".to_string()],
                },
            ),
            _ => self,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct GameState {
    pub objects: GameObjectState,
    pub camera: Vec2,
}

impl GameState {
    pub fn new() -> Self {
        GameState {
            objects: GameObjectState {
                balls: vec![Ball::new()],
                actuators: [
                    Actuator {
                        pos: Vec2::new(0.0, SCREEN_H - 60.0),
                    },
                    Actuator {
                        pos: Vec2::new(SCREEN_W - 60.0, SCREEN_H - 60.0),
                    },
                ],
            },
            camera: Vec2::new(0.0, 0.0),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct MenuState {
    pub selected: usize,
    pub options: Vec<String>,
}

#[derive(Debug, PartialEq)]
pub struct GameObjectState {
    pub balls: Vec<Ball>,
    pub actuators: [Actuator; 2],
}

#[derive(Debug, PartialEq)]
pub struct Ball {
    pub pos: Vec2,
    pub vel: Vec2,
}

#[derive(Debug, PartialEq)]
pub struct Actuator {
    pub pos: Vec2,
}

impl Ball {
    pub fn new() -> Self {
        Ball {
            pos: Vec2::new(0.0, 0.0),
            vel: Vec2::new(0.0, 0.0),
        }
    }
}
