use macroquad::prelude::*;

#[derive(Clone, PartialEq)]
pub enum EnemyVersion {
    Rock,
    Paper,
    Scissors,
}

#[derive(Clone, PartialEq)]
pub struct Enemy {
    pub x: f32,
    pub y: f32,
    pub speed: f32,
    pub version: EnemyVersion,
    sprite: Texture2D,
    scale: f32,
}

const DOWN_SCALE: f32 = 2.0;

impl Enemy {
    pub fn new(x: f32, y: f32, speed: f32, version: EnemyVersion, sprite: Texture2D) -> Self {
        sprite.set_filter(FilterMode::Linear);

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

    pub fn update_position(&mut self, enemies: &Vec<Enemy>) -> &Self {
        let mut closest_enemy = None;
        let mut closest_distance = f32::MAX;

        for enemy in enemies.iter() {
            if enemy.version == self.version {
                continue;
            }

            let distance = (enemy.x - self.x).powi(2) + (enemy.y - self.y).powi(2);

            if distance < closest_distance {
                closest_distance = distance;
                closest_enemy = Some(enemy);
            }
        }

        if let Some(enemy) = closest_enemy {
            let direction = vec2(enemy.x - self.x, enemy.y - self.y).normalize();
            self.x += direction.x * self.speed;
            self.y += direction.y * self.speed;
        }

        self
    }
}
