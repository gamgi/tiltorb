use crate::config::{SCALE, SCREEN_H, SCREEN_W};
use crate::game::balls::BALL_RADIUS;
use macroquad::math::{Vec2, Vec3};
use serde::Deserialize;
use std::fs::File;
use std::io::Read;

#[derive(Debug, PartialEq)]
#[allow(dead_code)]
pub enum State {
    Initial,
    Loading,
    Menu(GameState, MenuState),
    Game(GameState),
    Terminating,
}

#[derive(Debug, PartialEq)]
pub enum Event {
    AppInitialized,
    AppLoaded,
    MenuSelected(String),
    GameEnded,
    RoundCompleted,
    RoundLost,
}

pub const GRAVITY: (f32, f32, f32) = (0.0, 9.81, 0.0); // m/s
impl State {
    pub fn transition(self, event: Event) -> Self {
        match (self, event) {
            (State::Initial, Event::AppInitialized) => return State::Loading,
            (State::Loading, Event::AppLoaded) => {
                return State::Menu(GameState::new(), MenuState::main());
            }
            (State::Menu(game, _), Event::MenuSelected(item)) => match item.as_str() {
                "start" => {
                    return State::Game(game);
                }
                "quit" => State::Terminating,
                _ => unreachable!(),
            },
            (State::Game(_), Event::GameEnded) => {
                return State::Menu(GameState::new(), MenuState::main());
            }
            (State::Game(_), Event::RoundLost) => {
                println!("really lost");
                return State::Game(GameState::new());
            }
            (state, _) => state,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct GameState {
    pub objects: GameObjectState,
    pub camera: GameCameraState,
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
            camera: GameCameraState {
                pos: Vec2::new(0.0, 0.0),
                vel: Vec2::new(0.0, 0.0),
                rotation: 0.,
            },
            level: GameLevelState::from_file("assets/level_example.json"),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct MenuState {
    pub selected: usize,
    pub options: Vec<String>,
}

impl MenuState {
    pub fn main() -> Self {
        MenuState {
            selected: 0,
            options: vec!["Start".to_string(), "Quit".to_string()],
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct GameObjectState {
    pub balls: Vec<Ball>,
    pub actuators: [Actuator; 2],
}

#[derive(Debug, PartialEq)]
pub struct Ball {
    pub pos: Vec3,
    pub vel: Vec3,
    pub active: bool,
    pub forces: Vec<Vec3>,
    pub impulses: Vec<Vec3>,
    pub in_hole: bool,
}

#[derive(Debug, PartialEq)]
pub struct Actuator {
    pub pos: Vec2,
    pub vel: f32,
}

impl Ball {
    pub fn new() -> Self {
        Ball {
            pos: Vec3::new(
                SCREEN_W * 0.5 / SCALE,
                (SCREEN_H - 120.) / SCALE,
                BALL_RADIUS,
            ),
            vel: Vec3::new(0.0, 0.0, BALL_RADIUS),
            active: true,
            forces: vec![Vec3::from(GRAVITY)],
            impulses: vec![],
            in_hole: false,
        }
    }
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct GameLevelState {
    pub holes: Vec<Hole>,
    pub background_image: String,
}

impl GameLevelState {
    pub fn from_file(file_name: &str) -> Self {
        let mut file = File::open(file_name).unwrap();
        let mut data = String::new();
        file.read_to_string(&mut data).unwrap();

        let level: GameLevelState = serde_json::from_str(&data).expect("Could not parse JSON");
        level
    }
}

#[derive(Debug, PartialEq)]
pub struct GameCameraState {
    pub pos: Vec2,
    pub vel: Vec2,
    pub rotation: f32,
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct Hole {
    pub pos: Vec2,
    pub radius: f32,
}
