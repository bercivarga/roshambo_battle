use crate::asset_loader::AssetLoader;
use macroquad::prelude::*;

#[derive(Clone, PartialEq, Debug)]
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

const ENEMY_COUNT: usize = 1_000;
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

    pub fn update_position(&mut self, enemies: &mut Vec<Enemy>) -> Option<usize> {
        let mut closest_enemy = None;
        let mut closest_distance = f32::MAX;

        let mut hit = None;

        for enemy in enemies.iter() {
            let can_win = win_rules(self.version.to_owned(), enemy.version.to_owned());

            if !can_win {
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

            if !collision_detection(self, enemy) {
                return None;
            }

            self.scale += 0.05;
            self.speed -= 0.05;

            hit = Some(enemy.id);
        }

        hit
    }
}

// Basic rock-paper-scissors rules
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

// Creates hit-box
fn create_rect(el: &Enemy) -> Rect {
    Rect::new(
        el.x,
        el.y,
        el.sprite.width() as f32 * el.scale / DOWN_SCALE,
        el.sprite.height() as f32 * el.scale / DOWN_SCALE,
    )
}

pub fn generate_enemies(asset_loader: &AssetLoader) -> Vec<Enemy> {
    let mut enemies = vec![];
    for _ in 0..ENEMY_COUNT {
        // let x = rand::gen_range(-(MAP_SIZE / 2.0), MAP_SIZE / 2.0);
        // let y = rand::gen_range(-(MAP_SIZE / 2.0), MAP_SIZE / 2.0);
        let x = rand::gen_range(0.0, screen_width());
        let y = rand::gen_range(0.0, screen_height());
        let speed = rand::gen_range(0.5, 2.0);
        let version = match rand::gen_range(0, 3) {
            0 => EnemyVersion::Rock,
            1 => EnemyVersion::Paper,
            2 => EnemyVersion::Scissors,
            _ => unreachable!(),
        };

        let sprite = match version {
            EnemyVersion::Rock => asset_loader.rock,
            EnemyVersion::Paper => asset_loader.paper,
            EnemyVersion::Scissors => asset_loader.scissors,
        };

        let enemy = Enemy::new(x, y, speed, version, sprite, enemies.len());
        enemies.push(enemy);
    }

    enemies
}

pub fn update_all_enemies(enemies: &mut Vec<Enemy>) {
    // Need to create a copy so that we can pass reference without risking mutable references
    let mut new_enemies = enemies.to_vec();
    let mut hits = Vec::new();

    // Enemy update logic
    for enemy in enemies.iter_mut() {
        let hit = enemy.update_position(&mut new_enemies);

        if let Some(id) = hit {
            hits.push(id);
        }

        enemy.draw();
    }

    // Remove enemies that were hit
    enemies.retain(|enemy| !hits.contains(&enemy.id));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_win_rules() {
        assert_eq!(win_rules(EnemyVersion::Rock, EnemyVersion::Scissors), true);
        assert_eq!(win_rules(EnemyVersion::Paper, EnemyVersion::Rock), true);
        assert_eq!(win_rules(EnemyVersion::Scissors, EnemyVersion::Paper), true);
        assert_eq!(win_rules(EnemyVersion::Rock, EnemyVersion::Paper), false);
        assert_eq!(
            win_rules(EnemyVersion::Paper, EnemyVersion::Scissors),
            false
        );
        assert_eq!(win_rules(EnemyVersion::Scissors, EnemyVersion::Rock), false);
    }
}
