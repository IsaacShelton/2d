use macroquad::input::KeyCode::*;
use macroquad::prelude::*;

use crate::gamedata::GameData;
use crate::textures::Textures;

pub fn update(_textures: &Textures, gamedata: &mut GameData) {
    if is_key_down(Right) {
        gamedata.position += Vec2::new(4.0, 0.0);
    }

    if is_key_down(Left) {
        gamedata.position -= Vec2::new(4.0, 0.0);
    }

    if is_key_down(Down) {
        gamedata.position += Vec2::new(0.0, 4.0);
    }

    if is_key_down(Up) {
        gamedata.position -= Vec2::new(0.0, 4.0);
    }
}
