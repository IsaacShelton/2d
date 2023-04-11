use crate::gamedata::GameData;
use crate::textures::Textures;
use macroquad::prelude::*;

pub fn draw(textures: &Textures, gamedata: &GameData) {
    draw_texture(
        textures.example,
        gamedata.position.x,
        gamedata.position.y,
        WHITE,
    );
}
