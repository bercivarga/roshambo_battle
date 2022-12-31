mod asset_loader;
mod camera;
mod enemy;
mod gamestate;
mod player;

use asset_loader::AssetLoader;
use camera::Camera as PlayerCamera;
use enemy::{generate_enemies, update_all_enemies};
use gamestate::GameState;
use macroquad::prelude::*;
use player::Player;

// const MAP_SIZE: f32 = 2_000.0;

fn window_conf() -> Conf {
    Conf {
        window_title: "Roshambo Battle".to_owned(),
        fullscreen: false,
        high_dpi: true,
        window_width: 800,
        window_height: 600,
        ..Default::default()
    }
}
#[macroquad::main(window_conf)]
async fn main() {
    create_random_seed();

    let asset_loader = AssetLoader::new().await;
    let mut enemies = generate_enemies(&asset_loader);

    let mut player = Player {
        x: screen_width() / 2.0,
        y: screen_height() / 2.0,
        width: 20.0,
        height: 20.0,
        color: RED,
        speed: 3.0,
    };

    let mut camera = PlayerCamera::new();

    let mut gamestate = GameState::new();

    loop {
        clear_background(LIGHTGRAY);

        // handle reset
        if gamestate.needs_reset || is_key_pressed(KeyCode::R) {
            enemies = generate_enemies(&asset_loader);
            player.x = screen_width() / 2.0;
            player.y = screen_height() / 2.0;
            gamestate.needs_reset = false;
        }

        if gamestate.is_running() {
            // Player update logic
            player.update_position().draw();
            // Camera update logic
            camera.update(&player);
            // Enemy update logic
            update_all_enemies(&mut enemies);
        };

        // Gamestate update logic
        gamestate.win_checker(&enemies);
        gamestate.update();
        gamestate.draw();

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
