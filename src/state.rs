use crate::config::{SCALE, SCREEN_H, SCREEN_W};
use macroquad::math::Vec2;

#[derive(Debug, PartialEq)]
pub enum State {
    Initial,
    Loading,
    Menu(GameState, MenuState),
    Game(GameState),
    Terminating,
}

#[derive(Debug, PartialEq)]
pub enum Event {
    Initialized,
    Loaded,
    Selected(String),
}

impl State {
    pub fn transition(self, event: Event) -> Self {
        match (self, event) {
            (State::Initial, Event::Initialized) => return State::Loading,
            (State::Menu(game, _), Event::Selected(item)) => match item.as_str() {
                "start" => {
                    return State::Game(game);
                }
                "exit" => State::Terminating,
                _ => unreachable!(),
            },
            (State::Loading, Event::Loaded) => {
                return State::Menu(
                    GameState::new(),
                    MenuState {
                        selected: 0,
                        options: vec!["Start".to_string(), "Quit".to_string()],
                    },
                )
            }
            (state, _) => state,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct GameState {
    pub objects: GameObjectState,
    pub camera: Vec2,
    pub level: GameLevelState,
}

impl GameState {
    pub fn new() -> Self {
        GameState {
            objects: GameObjectState {
                balls: vec![Ball::new()],
                actuators: [
                    Actuator {
                        pos: Vec2::new(0.0, (SCREEN_H - 60.0) / SCALE),
                        vel: 0.0,
                    },
                    Actuator {
                        pos: Vec2::new((SCREEN_W - 60.0) / SCALE, (SCREEN_H - 60.0) / SCALE),
                        vel: 0.0,
                    },
                ],
            },
            camera: Vec2::new(0.0, 0.0),
            level: GameLevelState {
                holes: vec![
                    Hole {
                        pos: Vec2::new(0.1, 0.1),
                        radius: 0.04,
                    },
                    Hole {
                        pos: Vec2::new(0.2, 0.1),
                        radius: 0.04,
                    },
                    Hole {
                        pos: Vec2::new(0.2, 0.8),
                        radius: 0.05,
                    },
                    Hole {
                        pos: Vec2::new(0.9, 0.8),
                        radius: 0.05,
                    },
                    Hole {
                        pos: Vec2::new(1.0, 0.9),
                        radius: 0.04,
                    },
                ],
            },
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
    pub active: bool,
}

#[derive(Debug, PartialEq)]
pub struct Actuator {
    pub pos: Vec2,
    pub vel: f32,
}

impl Ball {
    pub fn new() -> Self {
        Ball {
            pos: Vec2::new(0.0, 0.0),
            vel: Vec2::new(0.0, 0.0),
            active: true,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct GameLevelState {
    pub holes: Vec<Hole>,
}

#[derive(Debug, PartialEq)]
pub struct Hole {
    pub pos: Vec2,
    pub radius: f32,
}
