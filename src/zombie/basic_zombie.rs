use macroquad::prelude::*;

use crate::constants::*;
use crate::plant::plant::Plant;
use crate::projectile::Instakill;
use crate::zombie::zombie::{Zombie, ZombieState};

pub struct BasicZombie {
    pub x: f32,
    pub y: f32,
    pub health: i32,
    pub speed: f32,
    pub state: ZombieState,
    pub attack_damage: i32,
    pub attack_cooldown: f32,
    pub last_attack_time: f64,
    pub slow_timer: f32,
    pub speed_multiplier: f32,
}

impl BasicZombie {
    pub fn new(y: f32) -> Self {
        Self {
            x: SCREEN_WIDTH,
            y,
            health: 100,
            speed: 20.0,
            state: ZombieState::Walking,
            attack_damage: 20,
            attack_cooldown: 1.0,
            last_attack_time: 0.0,
            slow_timer: 0.0,
            speed_multiplier: 1.0,
        }
    }
}

impl Zombie for BasicZombie {
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

    fn apply_slow(&mut self, duration: f32) {
        self.slow_timer = duration;
        self.speed_multiplier = 0.5;
    }

    fn is_dead(&self) -> bool {
        match self.state {
            ZombieState::Dead => true,
            _ => false,
        }
    }

    fn is_instakill(&mut self, tier: Instakill) {
        self.health = 0;
    }

    fn update(&mut self, plants: &mut Vec<Box<dyn Plant>>) {
        if self.state == ZombieState::Dead {
            return;
        }

        let dt = get_frame_time();

        // handle slow
        if self.slow_timer > 0.0 {
            self.slow_timer -= dt;
            if self.slow_timer <= 0.0 {
                self.speed_multiplier = 1.0;
            }
        }

        // find a plant in front
        let mut target: Option<&mut Box<dyn Plant>> = None;
        for plant in plants.iter_mut() {
            if (plant.y() - self.y).abs() < 40.0 && (plant.x() - self.x).abs() < 40.0 {
                target = Some(plant);
                break;
            }
        }

        if let Some(plant) = target {
            self.state = ZombieState::Attacking;
            let now = get_time();
            if now - self.last_attack_time >= self.attack_cooldown as f64 {
                self.last_attack_time = now;
                plant.take_damage(self.attack_damage);
            }
        } else {
            self.state = ZombieState::Walking;
            self.x -= self.speed * self.speed_multiplier * dt;
        }

        if self.health <= 0 {
            self.state = ZombieState::Dead;
        }
    }

    fn draw(&self) {
        if self.state == ZombieState::Dead {
            return;
        }

        let color = if self.slow_timer > 0.0 { BLUE } else { GREEN };
        draw_rectangle(self.x - 20.0, self.y - 40.0, 40.0, 80.0, color);
    }
}
