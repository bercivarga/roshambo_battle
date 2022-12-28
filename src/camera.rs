use crate::player::Player;
use macroquad::prelude::*;

pub struct Camera {
    pub instance: Camera2D,
}

impl Camera {
    pub fn new() -> Self {
        Self {
            instance: Camera2D::from_display_rect(Rect::new(
                0.0,
                0.0,
                screen_width(),
                screen_height(),
            )),
        }
    }

    pub fn update(&mut self, player: &Player) -> &mut Self {
        // Update camera
        self.instance.target = vec2(player.x, player.y);
        set_camera(&self.instance);

        self
    }
}
