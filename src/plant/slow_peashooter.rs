use crate::constants::*;
use crate::factory::projectile_factory::ProjectileKind;
use crate::plant::plant::{Plant, PlantAction};
use crate::zombie::zombie::Zombie;
use macroquad::prelude::*;

pub struct SlowPeashooter {
    pub x: f32,
    pub y: f32,
    pub cooldown: f32,
    pub timer: f32,
    pub health: i32,
    pub attack_range: f32,
    pub is_attacking: bool,
}

impl SlowPeashooter {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            x,
            y,
            cooldown: 1.5,
            timer: 0.0,
            health: 100,
            attack_range: 800.0, // basically whole row
            is_attacking: false,
        }
    }

    fn has_target(&self, zombies: &[Box<dyn Zombie>]) -> bool {
        for z in zombies {
            if (z.y() - self.y).abs() < TILE_HEIGHT / 2.0
                && z.x() > self.x
                && z.x() - self.x <= self.attack_range
                && !z.is_dead()
            {
                return true;
            }
        }
        false
    }
}

impl Plant for SlowPeashooter {
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

    fn update(&mut self, dt: f32, zombies: &[Box<dyn Zombie>]) -> Option<PlantAction> {
        self.timer -= dt;

        let has_target = self.has_target(zombies);
        self.is_attacking = has_target;

        if has_target && self.timer <= 0.0 {
            self.timer = self.cooldown;
            return Some(PlantAction::Shoot {
                kind: ProjectileKind::Slow,
                x: self.x + 25.0,
                y: self.y,
            });
        }
        None
    }

    fn draw(&self) {
        // Body
        draw_circle(self.x, self.y, 20.0, BLUE);
        // Head (pea cannon)
        draw_circle(self.x + 20.0, self.y, 10.0, DARKBLUE);

        // Eyes
        draw_circle(self.x - 6.0, self.y - 5.0, 2.0, BLACK);
        draw_circle(self.x + 6.0, self.y - 5.0, 2.0, BLACK);
    }
}
