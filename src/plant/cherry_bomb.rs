use crate::constants::*;
use crate::factory::projectile_factory::ProjectileKind;
use crate::plant::plant::{Plant, PlantAction};
use crate::projectile::projectile::Instakill;
use crate::zombie::zombie::Zombie;
use macroquad::prelude::*;

pub struct CherryBomb {
    pub x: f32,
    pub y: f32,
    pub timer: f32,
    pub health: i32,
}

impl CherryBomb {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            x,
            y,
            timer: 1.5,
            health: 100,
        }
    }
}

impl Plant for CherryBomb {
    fn x(&self) -> f32 {
        self.x
    }
    fn y(&self) -> f32 {
        self.y
    }
    fn health(&self) -> i32 {
        self.health
    }

    fn take_damage(&mut self, amount: i32) {
        self.health -= amount;
    }

    fn update(&mut self, dt: f32, zombies: &[Box<dyn Zombie>]) -> Option<PlantAction> {
        self.timer -= dt;

        if self.timer <= 0.0 {
            self.health = 0;
            return Some(PlantAction::Shoot {
                kind: ProjectileKind::Instakill {
                    radius: TILE_SIZE * 1.8,
                    tier: Instakill::Medium,
                },
                x: self.x,
                y: self.y,
            });
        }
        None
    }

    fn draw(&self) {
        // Left cherry
        draw_circle(self.x - 12.0, self.y, 14.0, RED);
        // Right cherry
        draw_circle(self.x + 12.0, self.y, 14.0, RED);

        // Cherry shine highlight
        draw_circle(self.x - 15.0, self.y - 5.0, 4.0, PINK);
        draw_circle(self.x + 9.0, self.y - 5.0, 4.0, PINK);

        // Green stem
        draw_line(
            self.x - 5.0,
            self.y - 14.0,
            self.x + 5.0,
            self.y - 22.0,
            3.0,
            DARKGREEN,
        );

        // Small leaf on stem
        draw_triangle(
            vec2(self.x + 5.0, self.y - 22.0),
            vec2(self.x + 12.0, self.y - 28.0),
            vec2(self.x, self.y - 26.0),
            GREEN,
        );
    }
}
