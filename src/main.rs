mod enemy;
mod player;

use enemy::{Enemy, EnemyVersion};
use macroquad::prelude::*;
use player::Player;
use std::fmt::Error;

const ENEMY_COUNT: usize = 20;

#[macroquad::main("BasicShapes")]
async fn main() {
    create_random_seed();

    let mut player = Player::new(
        screen_width() / 2.0,
        screen_height() / 2.0,
        20.0,
        20.0,
        RED,
        3.0,
    );

    let mut enemies = Vec::new();

    for _ in 0..ENEMY_COUNT {
        let x = rand::gen_range(0.0, screen_width());
        let y = rand::gen_range(0.0, screen_height());
        let speed = rand::gen_range(0.5, 2.0);
        let version = match rand::gen_range(0, 3) {
            0 => EnemyVersion::Rock,
            1 => EnemyVersion::Paper,
            2 => EnemyVersion::Scissors,
            _ => unreachable!(),
        };

        let enemy = Enemy::new(x, y, speed, version).await;
        enemies.push(enemy);
    }

    loop {
        clear_background(WHITE);

        player.update_position().draw();

        for enemy in enemies.iter_mut() {
            enemy.draw();
        }

        next_frame().await
    }
}

fn create_random_seed() {
    let time = get_time().to_string();

    let mut res = 0;

    for i in time.chars() {
        if i == '.' {
            continue;
        }

        let converted = i.to_digit(10).unwrap();
        res += converted as u64;
    }

    let _rng = rand::srand(res);
}
