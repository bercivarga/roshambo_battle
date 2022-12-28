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
    pub id: usize,
    pub should_remove: bool,
}

const DOWN_SCALE: f32 = 2.0;

impl Enemy {
    pub fn new(
        x: f32,
        y: f32,
        speed: f32,
        version: EnemyVersion,
        sprite: Texture2D,
        id: usize,
    ) -> Self {
        sprite.set_filter(FilterMode::Linear);

        Self {
            x,
            y,
            speed,
            version,
            sprite,
            scale: 0.5,
            id,
            should_remove: false,
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

    pub fn update_position(&mut self, enemies: &mut Vec<Enemy>) {
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

            if self.check_win(enemy) {
                self.should_remove = true;
            }
        }
    }

    pub fn check_win(&self, other: &Enemy) -> bool {
        if !collision_detection(self, other) {
            return false;
        }

        if self.version == other.version {
            return false;
        }

        if win_rules(self.version.to_owned(), other.version.to_owned()) {
            false
        } else {
            true
        }
    }
}

pub fn win_rules(ver1: EnemyVersion, ver2: EnemyVersion) -> bool {
    match (ver1, ver2) {
        (EnemyVersion::Rock, EnemyVersion::Scissors) => true,
        (EnemyVersion::Paper, EnemyVersion::Rock) => true,
        (EnemyVersion::Scissors, EnemyVersion::Paper) => true,
        _ => false,
    }
}

pub fn collision_detection(el1: &Enemy, el2: &Enemy) -> bool {
    let rect1 = create_rect(el1);
    let rect2 = create_rect(el2);
    rect1.overlaps(&rect2)
}

fn create_rect(el: &Enemy) -> Rect {
    Rect::new(
        el.x,
        el.y,
        el.sprite.width() as f32 * el.scale / DOWN_SCALE,
        el.sprite.height() as f32 * el.scale / DOWN_SCALE,
    )
}
