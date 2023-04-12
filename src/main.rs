use draw::draw;
use gamedata::GameData;
use macroquad::prelude::*;
use textures::Textures;
use update::update;

mod draw;
mod gamedata;
mod textures;
mod update;

fn get_macroquad_config() -> Conf {
    Conf {
        window_title: "{{name}}".into(),
        // fullscreen: true,
        ..Default::default()
    }
}

#[macroquad::main(get_macroquad_config)]
async fn main() {
    let textures: Textures = Textures::new().await;
    let mut gamedata: GameData = GameData::new();

    loop {
        clear_background(BLACK);

        update(&textures, &mut gamedata);
        draw(&textures, &gamedata);
        next_frame().await
    }
}
