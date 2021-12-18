use macroquad::math::Vec2;
#[derive(Debug, PartialEq)]
pub enum State {
    Menu(GameState),
    Game(GameState),
}

impl State {
    pub fn new() -> Self {
        State::Game(GameState {
            objects: GameObjectState {
                balls: vec![Ball::new()],
            },
        })
    }
}

#[derive(Debug, PartialEq)]
pub struct GameState {
    pub objects: GameObjectState,
}

#[derive(Debug, PartialEq)]
pub struct GameObjectState {
    pub balls: Vec<Ball>,
}

#[derive(Debug, PartialEq)]
pub struct Ball {
    pub pos: Vec2,
    pub vel: Vec2,
}

impl Ball {
    pub fn new() -> Self {
        Ball {
            pos: Vec2::new(0.0, 0.0),
            vel: Vec2::new(0.0, 0.0),
        }
    }
}
