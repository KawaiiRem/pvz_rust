use macroquad::prelude::*;
use crate::constants::*;
use crate::projectile::ProjectileType;
use crate::{projectile::Projectile, zombie::{Zombie, ZombieState}};

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum PlantType {
    Peashooter,
    SlowPeashooter,
    Sunflower,
}

impl PlantType {
    /// Sun cost for planting
    pub fn cost(&self) -> i32 {
        match self {
            PlantType::Peashooter => 100,
            PlantType::SlowPeashooter => 125,
            PlantType::Sunflower => 50,
        }
    }

    pub fn planting_cooldown(&self) -> f32 {
        match self {
            PlantType::Peashooter => 5.0,
            PlantType::SlowPeashooter => 7.5,
            PlantType::Sunflower => 7.5,
        }
    }
}

#[derive(Debug)]
pub enum PlantAction {
    Shoot(Projectile),
    ProduceSun { x: f32, y: f32 },
}

#[derive(Copy, Clone, Debug)]
pub struct Plant {
    pub plant_type: PlantType,
    pub x: f32,
    pub y: f32,
    pub cooldown: f32,
    pub timer: f32,
    pub health: i32,
    pub attack_range: f32,
    pub is_attacking: bool,   
}
impl Plant {
    pub fn is_dead(&self) -> bool {
        self.health <= 0
    }
}


impl Plant {
    pub fn new(plant_type: PlantType, x: f32, y: f32) -> Self {
        match plant_type {
            PlantType::Peashooter => Self {
                plant_type,
                x,
                y,
                cooldown: 1.5,
                timer: 0.0,
                health: 100,
                attack_range: 800.0,   // basically whole row
                is_attacking: false,
            },
            PlantType::SlowPeashooter => Self {
                plant_type,
                x,
                y,
                cooldown: 2.5,
                timer: 0.0,
                health: 100,
                attack_range: 800.0,   // basically whole row
                is_attacking: false,
            },
            PlantType::Sunflower => Self {
                plant_type,
                x,
                y,
                cooldown: 5.0,
                timer: 2.0,
                health: 80,
                attack_range: 0.0,    // doesn’t attack
                is_attacking: false,
            },
        }
    }

    fn has_target(&self, zombies: &[Zombie]) -> bool {
        for z in zombies {
            if (z.y - self.y).abs() < TILE_HEIGHT / 2.0 
            && z.x > self.x 
            && z.x - self.x <= self.attack_range 
            && z.state != ZombieState::Dead {
                return true;
            }
        }
        false
    }

    pub fn update(&mut self, dt: f32, zombies: &[Zombie]) -> Option<PlantAction> {
        self.timer -= dt;

        match self.plant_type {
            PlantType::Peashooter => {
                let has_target = self.has_target(zombies);
                self.is_attacking = has_target;

                if has_target && self.timer <= 0.0 {
                    self.timer = self.cooldown;
                    return Some(PlantAction::Shoot(Projectile::new(self.x + 25.0, self.y, ProjectileType::Normal)));
                }
            }
            PlantType::SlowPeashooter => {
                let has_target = self.has_target(zombies);
                self.is_attacking = has_target;

                if has_target && self.timer <= 0.0 {
                    self.timer = self.cooldown;
                    return Some(PlantAction::Shoot(Projectile::new(self.x + 25.0, self.y, ProjectileType::Slow)));
                }
            }
            PlantType::Sunflower => {
                if self.timer <= 0.0 {
                    self.timer = self.cooldown;
                    return Some(PlantAction::ProduceSun { x: self.x, y: self.y });
                }
            }
        }

        None
    }

    pub fn draw(&self) {
        match self.plant_type {
            PlantType::Peashooter => {
                draw_circle(self.x, self.y, 20.0, GREEN);
                draw_circle(self.x + 20.0, self.y, 10.0, DARKGREEN); // head
            }
            PlantType::SlowPeashooter => {
                draw_circle(self.x, self.y, 20.0, BLUE);
                draw_circle(self.x + 20.0, self.y, 10.0, DARKBLUE); // head
            }
            PlantType::Sunflower => {
                draw_circle(self.x, self.y, 18.0, YELLOW);
                draw_circle(self.x, self.y, 10.0, ORANGE); // center
            }
        }
    }
}
