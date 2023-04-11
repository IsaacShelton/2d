use macroquad::prelude::*;

pub struct Textures {
    pub example: Texture2D,
}

impl Textures {
    pub async fn new() -> Self {
        Textures {
            example: load_texture("assets/example.png").await.unwrap(),
        }
    }
}
