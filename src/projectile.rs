use macroquad::prelude::*;
use crate::constants::*;
use crate::zombie::{Zombie, ZombieState};

#[derive(Debug)]
pub struct Projectile {
    pub x: f32,
    pub y: f32,
    pub speed: f32,
    pub damage: i32,
    pub active: bool,
}

impl Projectile {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            x,
            y,
            speed: 200.0, // pixels per second
            damage: 20,
            active: true,
        }
    }

    pub fn update(&mut self, dt: f32, zombies: &mut Vec<Zombie>) {
        if !self.active {
            return;
        }

        self.x += self.speed * dt;

        // out of screen
        if self.x > screen_width() {
            self.active = false;
            return;
        }

        // check collisions
        for zombie in zombies.iter_mut() {
            if zombie.state == ZombieState::Dead {
                continue;
            }

            if (zombie.y - self.y).abs() < TILE_HEIGHT / 2.0 &&
               (zombie.x - self.x).abs() < 20.0 { // hitbox
                zombie.health -= self.damage;
                self.active = false;
                break;
            }
        }
    }

    pub fn draw(&self) {
        if self.active {
            draw_circle(self.x, self.y, 5.0, DARKGREEN);
        }
    }
}
