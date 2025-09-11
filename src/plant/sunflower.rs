use crate::plant::plant::{Plant, PlantAction};
use crate::zombie::zombie::Zombie;
use macroquad::prelude::*;

pub struct Sunflower {
    pub x: f32,
    pub y: f32,
    pub cooldown: f32,
    pub timer: f32,
    pub health: i32,
}

impl Sunflower {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            x,
            y,
            cooldown: 5.0,
            timer: 2.0,
            health: 80,
        }
    }
}

impl Plant for Sunflower {
    fn x(&self) -> f32 {
        self.x
    }
    fn y(&self) -> f32 {
        self.y
    }
    fn health(&self) -> i32 {
        self.health
    }
    fn is_tall(&self) -> bool {
        false
    }

    fn take_damage(&mut self, amount: i32) {
        self.health -= amount;
    }

    fn update(&mut self, dt: f32, _zombies: &[Box<dyn Zombie>]) -> Option<PlantAction> {
        self.timer -= dt;
        if self.timer <= 0.0 {
            self.timer = self.cooldown;
            return Some(PlantAction::ProduceSun {
                x: self.x,
                y: self.y,
            });
        }
        None
    }

    fn draw(&self) {
        // Petals
        draw_circle(self.x, self.y, 18.0, YELLOW);
        // Center
        draw_circle(self.x, self.y, 10.0, ORANGE);

        // Eyes
        draw_circle(self.x - 4.0, self.y - 2.0, 1.5, BLACK);
        draw_circle(self.x + 4.0, self.y - 2.0, 1.5, BLACK);

        // Smile
        draw_line(
            self.x - 4.0,
            self.y + 3.0,
            self.x + 4.0,
            self.y + 3.0,
            1.5,
            BLACK,
        );
    }
}
