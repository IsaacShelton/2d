use macroquad::prelude::*;

pub struct GameData {
    pub position: Vec2,
}

impl GameData {
    pub fn new() -> Self {
        GameData {
            position: Vec2::new(0.0, 0.0),
        }
    }
}
