mod asset_loader;
mod enemy;
mod player;

use asset_loader::AssetLoader;
use enemy::{update_all_enemies, Enemy, EnemyVersion};
use macroquad::prelude::*;
use player::Player;

const ENEMY_COUNT: usize = 1_000;

// const MAP_SIZE: f32 = 2_000.0;

fn window_conf() -> Conf {
    Conf {
        window_title: "Roshambo Battle".to_owned(),
        fullscreen: true,
        high_dpi: true,
        window_width: 1366,
        window_height: 768,
        ..Default::default()
    }
}
#[macroquad::main(window_conf)]
async fn main() {
    create_random_seed();

    let asset_loader = AssetLoader::new().await;

    let mut player = Player {
        x: screen_width() / 2.0,
        y: screen_height() / 2.0,
        width: 20.0,
        height: 20.0,
        color: RED,
        speed: 3.0,
    };

    let mut enemies = Vec::new();

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

    let mut camera =
        Camera2D::from_display_rect(Rect::new(0.0, 0.0, screen_width(), screen_height()));

    loop {
        clear_background(LIGHTGRAY);

        // Player update logic
        player.update_position().draw();

        // Update camera
        camera.target = vec2(player.x, player.y);
        set_camera(&camera);

        // Enemy update logic
        update_all_enemies(&mut enemies);

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
