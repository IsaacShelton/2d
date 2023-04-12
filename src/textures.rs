use macroquad::prelude::*;

pub struct Textures {
    pub example: Texture2D,
}

impl Textures {
    pub async fn new() -> Self {
        let example = load_texture("assets/example.png").await.unwrap();
        example.set_filter(FilterMode::Nearest);

        Textures { example }
    }
}
