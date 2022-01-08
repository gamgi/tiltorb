use crate::{
    config::{SCALE, SCREEN_H, SCREEN_W},
    game::balls::BALL_RADIUS,
    resources::Asset,
};
use macroquad::{
    math::{Vec2, Vec3},
    time::get_time,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq)]
#[allow(dead_code)]
pub enum State {
    Initial,
    Splash,
    Loading,
    Menu(GameState, MenuState),
    Game(GameState, DisplayState),
    Score(GameState, DisplayState),
    Editor(GameState, EditorState),
    Terminating,
}

#[derive(Debug, PartialEq)]
pub enum Event {
    AppInitialized,
    AppLoaded,
    SplashTimeout,
    MenuSelected(String),
    GameEnded,
    GameCompleted,
    RoundCompleted,
    RoundLost,
    EditorClosed,
}

pub const GRAVITY: (f32, f32, f32) = (0.0, 9.81, -0.1); // m/s
impl State {
    pub fn transition(self, event: Event) -> Self {
        match (self, event) {
            (State::Initial, Event::AppInitialized) => return State::Loading,
            (State::Loading, Event::AppLoaded) => return State::Splash,
            (State::Splash, Event::SplashTimeout) => {
                return State::Menu(GameState::new(), MenuState::main());
            }
            (State::Menu(_, _), Event::MenuSelected(item)) => match item.as_str() {
                "start" => {
                    return State::Game(GameState::load("level_example.json"), DisplayState::new());
                }
                "quit" => State::Terminating,
                "editor" => State::Editor(
                    GameState::load("level_new.json"),
                    EditorState { radius: 0.04 },
                ),
                _ => unreachable!(),
            },
            (State::Game(game, _), Event::GameCompleted) => {
                let game = game.next_round();
                let score = game.progress.score;
                return State::Score(
                    game,
                    DisplayState::messages(vec![
                        &format!("score {}", score),
                        &format!("score {}", score),
                        "game over",
                    ]),
                );
            }
            (State::Game(game, _) | State::Score(game, _), Event::GameEnded) => {
                return State::Menu(game, MenuState::main());
            }
            (State::Game(game, _), Event::RoundLost) => {
                let display = match game.progress.balls_left {
                    balls if balls == 0 => DisplayState::message("last ball"),
                    balls => DisplayState::message(&format!("{: >2} left", balls)),
                };
                return State::Game(game.reset_round(), display);
            }
            (State::Game(game, _), Event::RoundCompleted) => {
                return State::Game(game.next_round(), DisplayState::message("great"));
            }
            (State::Editor(game, _), Event::EditorClosed) => {
                return State::Game(game.reset_round(), DisplayState::new());
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
                pos: Vec2::new((SCREEN_W - 100.0) / SCALE, (SCREEN_H - 60.0) / SCALE),
                vel: 0.0,
            },
        ];
        self.objects.balls = vec![Ball::new()];
        self.progress.start_time = get_time() + 1.;
        self
    }

    pub fn next_round(mut self) -> Self {
        self.progress.goal_index += 1;
        self.progress.score += self.progress.time() as u16;
        self.reset_round()
    }

    pub fn get_goal_hole(&self) -> usize {
        *self.level.goals.get(self.progress.goal_index).unwrap_or(&0)
    }
}

impl Default for GameState {
    fn default() -> Self {
        GameState {
            progress: GameProgressState {
                start_time: get_time() + 1.,
                goal_index: 0,
                score: 0,
                balls_left: 4,
            },
            objects: GameObjectState {
                balls: vec![Ball::new()],
                actuators: [
                    Actuator {
                        pos: Vec2::new(0.0, (SCREEN_H - 60.0) / SCALE),
                        vel: 0.0,
                    },
                    Actuator {
                        pos: Vec2::new((SCREEN_W - 100.0) / SCALE, (SCREEN_H - 60.0) / SCALE),
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
    pub start_time: f64,
    pub score: u16,
    pub balls_left: u16,
}

#[derive(Debug, PartialEq)]
pub struct DisplayState {
    pub message: Option<String>,
    pub messages: Vec<String>,
    pub start_time: f64,
}

impl DisplayState {
    pub fn new() -> Self {
        DisplayState {
            message: None,
            messages: vec![],
            start_time: 0.,
        }
    }
    pub fn message(message: &str) -> Self {
        DisplayState {
            message: Some(message.to_string()),
            messages: vec![],
            start_time: get_time(),
        }
    }
    pub fn messages(messages: Vec<&str>) -> Self {
        DisplayState {
            message: None,
            messages: messages.iter().map(|msg| msg.to_string()).collect(),
            start_time: 0.,
        }
    }
}

impl GameProgressState {
    pub fn time(&self) -> f64 {
        99. - f64::max(0., get_time() - self.start_time).round()
    }
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
