use crate::enemy::{Enemy, EnemyVersion};
use macroquad::prelude::*;

#[derive(Debug)]
pub struct GameState {
    is_running: bool,
    winner: Option<EnemyVersion>,
    is_start_screen: bool,
    is_end_screen: bool,
    pub needs_reset: bool,
}

impl GameState {
    pub fn new() -> Self {
        Self {
            is_running: false,
            winner: None,
            is_start_screen: true,
            is_end_screen: false,
            needs_reset: false,
        }
    }

    pub fn draw(&self) -> &Self {
        if self.is_start_screen {
            draw_text("Press Space to Start", 200.0, 200.0, 40.0, BLACK);
        }

        if self.is_end_screen {
            let winner = match self.winner {
                Some(EnemyVersion::Rock) => "Rock",
                Some(EnemyVersion::Paper) => "Paper",
                Some(EnemyVersion::Scissors) => "Scissors",
                None => "None",
            };

            draw_text(
                &format!("Team {:?} won!", winner),
                200.0,
                200.0,
                40.0,
                BLACK,
            );

            draw_text("Press Space to Restart", 200.0, 200.0 + 80.0, 40.0, BLACK);
        }

        self
    }

    pub fn win_checker(&mut self, enemies: &Vec<Enemy>) {
        if !self.is_running || self.is_end_screen || self.is_start_screen || enemies.len() == 0 {
            return;
        }

        let mut all_same = true;
        let mut enemy_type = &enemies[0].version;
        for enemy in enemies.iter() {
            if enemy.version != enemy_type.to_owned() {
                all_same = false;
            }
        }

        if all_same {
            self.set_winner(enemy_type.to_owned());
            self.set_end_screen(true);
            self.set_running(false);
        }
    }

    pub fn update(&mut self) -> &mut Self {
        if self.is_start_screen {
            if is_key_pressed(KeyCode::Space) {
                self.is_start_screen = false;
                self.is_running = true;
            }
        }

        if self.is_end_screen {
            if is_key_pressed(KeyCode::Space) {
                self.is_end_screen = false;
                self.is_running = true;
                self.winner = None;
                self.needs_reset = true;
            }
        }

        self
    }

    pub fn set_winner(&mut self, winner: EnemyVersion) -> &mut Self {
        self.winner = Option::from(winner);
        self
    }

    pub fn set_running(&mut self, running: bool) -> &mut Self {
        self.is_running = running;
        self
    }

    pub fn set_start_screen(&mut self, start_screen: bool) -> &mut Self {
        self.is_start_screen = start_screen;
        self
    }

    pub fn set_end_screen(&mut self, end_screen: bool) -> &mut Self {
        self.is_end_screen = end_screen;
        self
    }

    pub fn is_running(&self) -> bool {
        self.is_running
    }
}
