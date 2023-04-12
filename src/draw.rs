use crate::gamedata::GameData;
use crate::textures::Textures;
use macroquad::prelude::*;

pub fn draw(textures: &Textures, gamedata: &GameData) {
    draw_text("Hello World", 10.0, 60.0, 40.0, RED);

    draw_texture(
        textures.example,
        gamedata.position.x,
        gamedata.position.y,
        WHITE,
    );

    draw_texture_ex(
        textures.example,
        gamedata.position.x,
        gamedata.position.y,
        WHITE,
        DrawTextureParams {
            dest_size: Some(vec2(128.0, 128.0)),
            ..Default::default()
        },
    )
}
