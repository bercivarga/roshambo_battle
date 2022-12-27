use macroquad::prelude::*;

pub enum EnemyVersion {
    Rock,
    Paper,
    Scissors,
}

pub struct Enemy {
    pub x: f32,
    pub y: f32,
    pub speed: f32,
    pub version: EnemyVersion,
    sprite: Texture2D,
    scale: f32,
}

const DOWN_SCALE: f32 = 10.0;

impl Enemy {
    pub async fn new(x: f32, y: f32, speed: f32, version: EnemyVersion) -> Self {
        let sprite = match version {
            EnemyVersion::Rock => load_texture("assets/rock.png").await,
            EnemyVersion::Paper => load_texture("assets/paper.png").await,
            EnemyVersion::Scissors => load_texture("assets/scissors.png").await,
        }
        .unwrap();

        Self {
            x,
            y,
            speed,
            version,
            sprite,
            scale: 0.5,
        }
    }

    pub fn draw(&self) -> &Self {
        let draw_texture_params = DrawTextureParams {
            dest_size: Some(vec2(
                self.sprite.width() as f32 * self.scale / DOWN_SCALE,
                self.sprite.height() as f32 * self.scale / DOWN_SCALE,
            )),
            ..Default::default()
        };

        draw_texture_ex(self.sprite, self.x, self.y, WHITE, draw_texture_params);

        self
    }

    pub fn _update_position(&mut self) -> &Self {
        todo!("Update the position of the enemy");
        self
    }
}
