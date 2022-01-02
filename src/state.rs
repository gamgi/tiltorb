use crate::{
    config::{SCALE, SCREEN_H, SCREEN_W},
    game::balls::BALL_RADIUS,
    resources::Asset,
};
use macroquad::math::{Vec2, Vec3};
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq)]
#[allow(dead_code)]
pub enum State {
    Initial,
    Loading,
    Menu(GameState, MenuState),
    Game(GameState),
    Editor(GameState, EditorState),
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
    EditorClosed,
}

pub const GRAVITY: (f32, f32, f32) = (0.0, 9.81, -0.1); // m/s
impl State {
    pub fn transition(self, event: Event) -> Self {
        match (self, event) {
            (State::Initial, Event::AppInitialized) => return State::Loading,
            (State::Loading, Event::AppLoaded) => {
                return State::Menu(GameState::new(), MenuState::main());
            }
            (State::Menu(_, _), Event::MenuSelected(item)) => match item.as_str() {
                "start" => {
                    return State::Game(GameState::load("level_example.json"));
                }
                "quit" => State::Terminating,
                "editor" => State::Editor(
                    GameState::load("level_new.json"),
                    EditorState { radius: 0.04 },
                ),
                _ => unreachable!(),
            },
            (State::Game(_), Event::GameEnded) => {
                return State::Menu(GameState::load("level_example.json"), MenuState::main());
            }
            (State::Game(game), Event::RoundLost) => {
                return State::Game(game.reset_round());
            }
            (State::Game(game), Event::RoundCompleted) => {
                return State::Game(game.next_round());
            }
            (State::Editor(game, _), Event::EditorClosed) => {
                return State::Game(game.reset_round());
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
    pub progress: GameProgressState,
}

impl GameState {
    pub fn new() -> Self {
        GameState {
            level: GameLevelState::new(),
            ..Default::default()
        }
    }

    pub fn load(level_file: &str) -> Self {
        GameState {
            level: GameLevelState::from_file(level_file),
            ..Default::default()
        }
    }

    pub fn reset_round(mut self) -> Self {
        self.objects.actuators = [
            Actuator {
                pos: Vec2::new(0.0, (SCREEN_H - 60.0) / SCALE),
                vel: 0.0,
            },
            Actuator {
                pos: Vec2::new((SCREEN_W - 60.0) / SCALE, (SCREEN_H - 60.0) / SCALE),
                vel: 0.0,
            },
        ];
        self.objects.balls = vec![Ball::new()];
        self
    }

    pub fn next_round(mut self) -> Self {
        self.progress.goal_index += 1;
        self.reset_round()
    }

    pub fn get_goal_hole(&self) -> usize {
        *self.level.goals.get(self.progress.goal_index).unwrap_or(&0)
    }
}

impl Default for GameState {
    fn default() -> Self {
        GameState {
            progress: GameProgressState { goal_index: 0 },
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
            level: GameLevelState::new(),
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
            options: vec![
                "Start".to_string(),
                // "Editor".to_string(),
                "Quit".to_string(),
            ],
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
    pub in_hole: Option<usize>,
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
            in_hole: None,
        }
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct GameLevelState {
    pub background_image: String,
    pub goals: Vec<usize>,
    pub holes: Vec<Hole>,
    #[serde(skip)]
    pub level_file: String,
}

impl GameLevelState {
    pub fn new() -> Self {
        GameLevelState {
            background_image: "level_example.png".to_string(),
            goals: Vec::new(),
            holes: Vec::new(),
            level_file: "level_new.json".to_string(),
        }
    }

    pub fn from_file(level_file: &str) -> Self {
        let data = Asset::get(level_file)
            .expect(&format!("Could not load level \"{}\"", level_file))
            .data;
        let data = serde_json::from_slice(&data).expect("Could not parse JSON");

        GameLevelState {
            level_file: level_file.to_string(),
            ..data
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct GameCameraState {
    pub pos: Vec2,
    pub vel: Vec2,
    pub rotation: f32,
}

#[derive(Debug, PartialEq)]
pub struct GameProgressState {
    pub goal_index: usize,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Hole {
    pub pos: Vec2,
    pub radius: f32,
}

#[derive(Debug, PartialEq)]
pub struct EditorState {
    pub radius: f32,
}
