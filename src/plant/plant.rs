use crate::{factory::projectile_factory::ProjectileKind, zombie::zombie::Zombie};
use macroquad::prelude::*;

#[derive(Debug)]
pub enum PlantAction {
    Shoot {
        kind: ProjectileKind,
        x: f32,
        y: f32,
    },
    ProduceSun {
        x: f32,
        y: f32,
    },
}

pub trait Plant {
    fn x(&self) -> f32;
    fn y(&self) -> f32;
    fn health(&self) -> i32;
    fn take_damage(&mut self, amount: i32);

    fn update(&mut self, dt: f32, zombies: &[Box<dyn Zombie>]) -> Option<PlantAction>;
    fn draw(&self);

    fn is_dead(&self) -> bool {
        self.health() <= 0
    }
}
