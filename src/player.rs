use macroquad::prelude::*;

pub struct Player {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub color: Color,
    pub speed: f32,
}

impl Player {
    pub fn draw(&self) {
        draw_rectangle(self.x, self.y, self.width, self.height, self.color);
        draw_text("You", self.x - 3.0, self.y + 32.0, 20.0, BLACK);
    }

    pub fn update_position(&mut self) -> &Self {
        let mut direction = Vec2::new(0.0, 0.0);

        if is_key_down(KeyCode::W) {
            direction.y -= 1.0;
        }
        if is_key_down(KeyCode::S) {
            direction.y += 1.0;
        }
        if is_key_down(KeyCode::A) {
            direction.x -= 1.0;
        }
        if is_key_down(KeyCode::D) {
            direction.x += 1.0;
        }

        direction *= self.speed;

        self.x += direction.x;
        self.y += direction.y;

        self
    }
}
