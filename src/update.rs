use macroquad::input::KeyCode::*;
use macroquad::prelude::*;

use crate::gamedata::GameData;
use crate::textures::Textures;

pub fn update(_textures: &Textures, gamedata: &mut GameData) {
    let dt = get_frame_time() * 60.0;
    let amount = 8.0 * dt;

    if is_key_down(Right) {
        gamedata.position += Vec2::new(amount, 0.0);
    }

    if is_key_down(Left) {
        gamedata.position -= Vec2::new(amount, 0.0);
    }

    if is_key_down(Down) {
        gamedata.position += Vec2::new(0.0, amount);
    }

    if is_key_down(Up) {
        gamedata.position -= Vec2::new(0.0, amount);
    }
}
