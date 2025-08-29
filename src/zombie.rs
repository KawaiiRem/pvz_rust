use macroquad::prelude::*;

use crate::plant::{Plant, PlantType};
use crate::constants::*;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ZombieState {
    Walking,
    Attacking,
    Dead,
}

pub struct Zombie {
    pub x: f32,
    pub y: f32,
    pub lane: usize,       // row index
    pub health: i32,
    pub state: ZombieState,
    pub speed: f32,
    pub attack_damage: i32,
    pub attack_cooldown: f32,
    pub last_attack_time: f64,
}

impl Zombie {
    pub fn new(lane: usize) -> Self {
        let y = lane as f32 * TILE_SIZE + TILE_SIZE / 2.0 + UI_BAR_HEIGHT;
        Self {
            x: SCREEN_WIDTH,
            y,
            lane,
            health: 100,
            state: ZombieState::Walking,
            speed: 20.0,
            attack_damage: 20,
            attack_cooldown: 1.0,
            last_attack_time: 0.0,
        }
    }

    pub fn update(&mut self, plants: &mut Vec<Plant>) {
        if self.state == ZombieState::Dead {
            return;
        }

        // Find plant in same lane and within attack range
        let mut target: Option<&mut Plant> = None;
        for plant in plants.iter_mut() {
            if (plant.y - self.y).abs() < TILE_HEIGHT / 2.0 {
                if (plant.x - self.x).abs() < 40.0 { // bite range
                    target = Some(plant);
                    break;
                }
            }
        }

        if let Some(plant) = target {
            self.state = ZombieState::Attacking;
            let now = get_time();
            if now - self.last_attack_time >= self.attack_cooldown as f64 {
                self.last_attack_time = now;
                plant.health -= self.attack_damage;
            }
        } else {
            self.state = ZombieState::Walking;
            self.x -= self.speed * get_frame_time();
        }

        // Check zombie dead
        if self.health <= 0 {
            self.state = ZombieState::Dead;
        }
    }

    pub fn draw(&self) {
        if self.state == ZombieState::Dead {
            return;
        }

        let color = match self.state {
            ZombieState::Walking => GREEN,
            ZombieState::Attacking => RED,
            ZombieState::Dead => DARKGRAY,
        };

        draw_rectangle(self.x - 20.0, self.y - 40.0, 40.0, 80.0, color);
    }
}
