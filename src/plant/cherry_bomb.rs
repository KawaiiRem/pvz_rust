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

    fn is_tall(&self) -> bool {
        false
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
        // Growing effect: scale grows as timer approaches 0
        let max_scale = 1.5;
        let min_scale = 1.0;
        let scale = if self.timer > 0.0 {
            let progress = (1.5 - self.timer) / 1.5; // progress from 0 → 1
            min_scale + (max_scale - min_scale) * progress.clamp(0.0, 1.0)
        } else {
            max_scale
        };

        let left_x = self.x - 12.0;
        let right_x = self.x + 12.0;
        let y = self.y;
        let radius = 14.0 * scale;

        // Left cherry
        draw_circle(left_x, y, radius, RED);
        // Right cherry
        draw_circle(right_x, y, radius, RED);

        // Cherry shine highlight
        draw_circle(left_x - 3.0 * scale, y - 5.0 * scale, 4.0 * scale * 0.3, PINK);
        draw_circle(right_x - 3.0 * scale, y - 5.0 * scale, 4.0 * scale * 0.3, PINK);

        // Green stem
        draw_line(
            self.x - 5.0 * scale,
            self.y - 14.0 * scale,
            self.x + 5.0 * scale,
            self.y - 22.0 * scale,
            3.0 * scale,
            DARKGREEN,
        );

        // Leaf on stem
        draw_triangle(
            vec2(self.x + 5.0 * scale, self.y - 22.0 * scale),
            vec2(self.x + 12.0 * scale, self.y - 28.0 * scale),
            vec2(self.x, self.y - 26.0 * scale),
            GREEN,
        );

        // Faces on cherries
        let eye_offset_y = -2.0 * scale;

        // Left cherry face
        draw_circle(left_x - 2.0, y + eye_offset_y, 2.0 * scale * 0.3, BLACK);
        draw_circle(left_x + 2.0, y + eye_offset_y, 2.0 * scale * 0.3, BLACK);
        draw_line(
            left_x - 2.0,
            y + 4.0 * scale * 0.3,
            left_x + 2.0,
            y + 4.0 * scale * 0.3,
            1.0 * scale * 0.3,
            BLACK,
        );

        // Right cherry face
        draw_circle(right_x - 2.0, y + eye_offset_y, 2.0 * scale * 0.3, BLACK);
        draw_circle(right_x + 2.0, y + eye_offset_y, 2.0 * scale * 0.3, BLACK);
        draw_line(
            right_x - 2.0,
            y + 4.0 * scale * 0.3,
            right_x + 2.0,
            y + 4.0 * scale * 0.3,
            1.0 * scale * 0.3,
            BLACK,
        );
    }
}
