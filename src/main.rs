mod asset_loader;
mod enemy;
mod player;

use asset_loader::AssetLoader;
use enemy::{Enemy, EnemyVersion};
use macroquad::prelude::*;
use player::Player;

const ENEMY_COUNT: usize = 20;

fn window_conf() -> Conf {
    Conf {
        window_title: "RPS Battle".to_owned(),
        fullscreen: false,
        window_resizable: true,
        window_width: 1000,
        window_height: 800,
        ..Default::default()
    }
}
#[macroquad::main(window_conf)]
async fn main() {
    create_random_seed();

    let asset_loader = AssetLoader::new().await;

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

        let sprite = match version {
            EnemyVersion::Rock => asset_loader.rock,
            EnemyVersion::Paper => asset_loader.paper,
            EnemyVersion::Scissors => asset_loader.scissors,
        };

        let enemy = Enemy::new(x, y, speed, version, sprite);
        enemies.push(enemy);
    }

    loop {
        clear_background(WHITE);

        player.update_position().draw();

        for enemy in enemies.iter_mut() {
            enemy.update_position().draw();
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
