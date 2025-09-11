use crate::constants::*;
use crate::factory::projectile_factory::ProjectileKind;
use crate::plant::plant::{Plant, PlantAction};
use crate::projectile::projectile::Instakill;
use crate::zombie::zombie::Zombie;
use macroquad::prelude::*;

pub struct PotatoMine {
    pub x: f32,
    pub y: f32,
    pub cooldown: f32,
    pub timer: f32,
    pub health: i32,
    pub attack_range: f32,
    pub is_attacking: bool,
}

impl PotatoMine {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            x,
            y,
            cooldown: 14.0,
            timer: 14.0,
            health: 100,
            attack_range: TILE_SIZE / 2.0,
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

impl Plant for PotatoMine {
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

        let has_target = self.has_target(zombies);
        self.is_attacking = has_target;

        if has_target && self.timer <= 0.0 {
            self.timer = self.cooldown;
            self.health = 0;
            return Some(PlantAction::Shoot{
                kind: ProjectileKind::Instakill { radius: TILE_SIZE * 1.5, tier: Instakill::Low },
                x: self.x,
                y: self.y,
            });
        }
        None
    }

    fn draw(&self) {
        if self.timer > 0.0 {
            // not armed yet → buried look
            draw_circle(self.x, self.y, 15.0, BROWN);
            draw_circle(self.x + 5.0, self.y, 10.0, DARKBROWN);
        } else {
            // armed → full potato mine
            draw_circle(self.x, self.y, 20.0, ORANGE);
            draw_circle(self.x, self.y, 18.0, BROWN);
            
            draw_circle(self.x, self.y - 7.0, 3.0, WHITE);
            draw_circle(self.x, self.y + 7.0, 3.0, WHITE);
            draw_circle(self.x, self.y - 7.0, 1.5, BLACK);
            draw_circle(self.x, self.y + 7.0, 1.5, BLACK);

            draw_circle(self.x + 12.0, self.y - 8.0, 4.0, RED);
        }
    }
}
