use crate::{plant::plant::Plant, projectile::projectile::Instakill};

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum ZombieState {
    Walking,
    Attacking,
    Dead,
}

pub trait Zombie {
    fn x(&self) -> f32;
    fn y(&self) -> f32;
    fn health(&self) -> i32;
    fn is_dead(&self) -> bool;

    fn take_damage(&mut self, amount: i32);
    fn apply_slow(&mut self, duration: f32);
    fn is_instakill(&mut self, tier: Instakill);

    fn update(&mut self, plants: &mut Vec<Box<dyn Plant>>);
    fn draw(&self);
}
