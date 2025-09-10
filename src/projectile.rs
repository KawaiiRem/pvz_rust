use crate::{constants::*, zombie::zombie::Zombie};
use macroquad::prelude::*;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ProjectileType {
    Normal,
    Slow,
}

#[derive(Debug)]
pub struct Projectile {
    pub x: f32,
    pub y: f32,
    pub speed: f32,
    pub damage: i32,
    pub active: bool,
    pub projectile_type: ProjectileType,
}

impl Projectile {
    pub fn new(x: f32, y: f32, projectile_type: ProjectileType) -> Self {
        match projectile_type {
            ProjectileType::Normal => Self {
                x,
                y,
                speed: 300.0, // pixels per second
                damage: 20,
                active: true,
                projectile_type,
            },
            ProjectileType::Slow => Self {
                x,
                y,
                speed: 300.0,
                damage: 10,
                active: true,
                projectile_type,
            },
        }
    }

    pub fn update(&mut self, dt: f32, zombies: &mut Vec<Box<dyn Zombie>>) {
        if !self.active {
            return;
        }

        // move projectile
        self.x += self.speed * dt;

        // out of screen
        if self.x > screen_width() {
            self.active = false;
            return;
        }

        // check collisions against every zombie
        for zombie in zombies.iter_mut() {
            if zombie.is_dead() {
                continue;
            }

            let same_row = (zombie.y() - self.y).abs() < TILE_HEIGHT / 2.0;
            let hitbox = (zombie.x() - self.x).abs() < 20.0;

            if same_row && hitbox {
                zombie.take_damage(self.damage);

                if self.projectile_type == ProjectileType::Slow {
                    zombie.apply_slow(2.5); // <-- requires Zombie trait to have this
                }

                self.active = false;
                break;
            }
        }
    }

    pub fn draw(&self) {
        if !self.active {
            return;
        }

        match self.projectile_type {
            ProjectileType::Normal => draw_circle(self.x, self.y, 5.0, DARKGREEN),
            ProjectileType::Slow => draw_circle(self.x, self.y, 6.0, DARKBLUE),
        }
    }
}
