use super::projectile::{Instakill, Projectile};
use crate::zombie::zombie::Zombie;
use macroquad::prelude::*;

pub struct InstakillExplode {
    pub x: f32,
    pub y: f32,
    pub radius: f32,
    pub tier: Instakill,
    pub lifetime: f32,
    pub active: bool,
}

impl InstakillExplode {
    pub fn new(x: f32, y: f32, radius: f32, tier: Instakill) -> Self {
        Self {
            x,
            y,
            radius,
            tier,
            lifetime: 0.6,
            active: true,
        }
    }
}

impl Projectile for InstakillExplode {
    fn is_active(&self) -> bool {
        self.active
    }

    fn update(&mut self, dt: f32, zombies: &mut Vec<Box<dyn Zombie>>) {
        self.lifetime -= dt;

        for z in zombies.iter_mut() {
            if z.is_dead() {
                continue;
            }
            let dx = z.x() - self.x;
            let dy = z.y() - self.y;
            if (dx * dx + dy * dy).sqrt() <= self.radius {
                z.is_instakill(self.tier);
            }
        }

        if self.lifetime <= 0.0 {
            self.active = false;
        }
    }

    fn draw(&self) {
        if self.active {
            let alpha = (self.lifetime * 2.0).min(1.0);
            let color = Color::new(1.0, 0.2, 0.2, alpha);
            draw_circle(self.x, self.y, self.radius, color);
            draw_circle_lines(self.x, self.y, self.radius + 10.0, 3.0, ORANGE);
        }
    }
}
