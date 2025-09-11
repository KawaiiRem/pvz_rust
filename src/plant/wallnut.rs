use crate::plant::plant::{Plant, PlantAction};
use crate::zombie::zombie::Zombie;
use macroquad::prelude::*;

pub struct Wallnut {
    pub x: f32,
    pub y: f32,
    pub current_health: i32,
    max_health: i32,
}

impl Wallnut {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            x,
            y,
            current_health: 500,
            max_health: 500,
        }
    }
}

impl Plant for Wallnut {
    fn x(&self) -> f32 {
        self.x
    }
    fn y(&self) -> f32 {
        self.y
    }
    fn health(&self) -> i32 {
        self.current_health
    }

    fn take_damage(&mut self, amount: i32) {
        self.current_health -= amount;
    }

    fn update(&mut self, dt: f32, zombies: &[Box<dyn Zombie>]) -> Option<PlantAction> {
        None
    }

    fn draw(&self) {
        draw_ellipse(self.x, self.y, 16.0, 22.0, 0.0, BROWN);
        draw_ellipse(self.x, self.y, 14.0, 20.0, 0.0, DARKBROWN);

        draw_circle(self.x - 6.0, self.y - 6.0, 3.5, WHITE);
        draw_circle(self.x + 6.0, self.y - 6.0, 3.5, WHITE);
        draw_circle(self.x - 6.0, self.y - 6.0, 1.8, BLACK);
        draw_circle(self.x + 6.0, self.y - 6.0, 1.8, BLACK);

        draw_line(
            self.x - 6.0,
            self.y + 7.0,
            self.x + 6.0,
            self.y + 7.0,
            2.0,
            BLACK,
        );

        // --- Damage cracks if HP is low ---
        let hp_ratio = self.current_health as f32 / self.max_health as f32;
        if hp_ratio < 0.66 {
            // light cracks
            draw_line(self.x - 8.0, self.y, self.x - 2.0, self.y + 6.0, 1.5, BLACK);
        }
        if hp_ratio < 0.33 {
            // heavier cracks
            draw_line(
                self.x + 4.0,
                self.y - 4.0,
                self.x + 10.0,
                self.y + 2.0,
                1.5,
                BLACK,
            );
            draw_line(
                self.x - 3.0,
                self.y + 10.0,
                self.x + 2.0,
                self.y + 14.0,
                1.5,
                BLACK,
            );
        }
    }
}
