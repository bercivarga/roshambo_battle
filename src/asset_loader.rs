use macroquad::texture::Texture2D;

pub struct AssetLoader {
    pub rock: Texture2D,
    pub paper: Texture2D,
    pub scissors: Texture2D,
}

impl AssetLoader {
    pub async fn new() -> Self {
        Self {
            rock: macroquad::texture::load_texture("assets/rock.png")
                .await
                .unwrap(),
            paper: macroquad::texture::load_texture("assets/paper.png")
                .await
                .unwrap(),
            scissors: macroquad::texture::load_texture("assets/scissors.png")
                .await
                .unwrap(),
        }
    }
}
