use crate::zombie::zombie::Zombie;
use macroquad::prelude::*;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Instakill {
    Low,
    Medium,
    High,
}

impl Instakill {
    // if zombie resists instakill, it loses this fraction of its max health instead
    pub fn hp_fraction(&self) -> f32 {
        match self {
            Instakill::Low => 0.10,
            Instakill::Medium => 0.25,
            Instakill::High => 0.50,
        }
    }
}

pub trait Projectile {
    fn is_active(&self) -> bool;

    fn update(&mut self, dt: f32, zombies: &mut Vec<Box<dyn Zombie>>);
    fn draw(&self);
}
