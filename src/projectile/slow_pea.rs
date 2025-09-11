use crate::{constants::*, zombie::zombie::Zombie};
use macroquad::prelude::*;
use super::projectile::Projectile;

pub struct SlowPea {
    pub x: f32,
    pub y: f32,
    pub speed: f32,
    pub damage: i32,
    pub active: bool,
}

impl SlowPea {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            x,
            y,
            speed: 300.0,
            damage: 10,
            active: true,
        }
    }
}

impl Projectile for SlowPea {
    fn x(&self) -> f32 { self.x }
    fn y(&self) -> f32 { self.y }
    fn is_active(&self) -> bool { self.active }

    fn update(&mut self, dt: f32, zombies: &mut Vec<Box<dyn Zombie>>) {
        self.x += self.speed * dt;

        if self.x > screen_width() {
            self.active = false;
            return;
        }

        for z in zombies.iter_mut() {
            if z.is_dead() { continue; }
            let same_row = (z.y() - self.y).abs() < TILE_HEIGHT / 2.0;
            let hitbox = (z.x() - self.x).abs() < 20.0;
            if same_row && hitbox {
                z.take_damage(self.damage);
                z.apply_slow(2.5);
                self.active = false;
                break;
            }
        }
    }

    fn draw(&self) {
        if self.active {
            draw_circle(self.x, self.y, 6.0, DARKBLUE);
        }
    }
}
