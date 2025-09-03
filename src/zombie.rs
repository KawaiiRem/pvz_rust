use macroquad::prelude::*;

use crate::plant::Plant;
use crate::constants::*;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ZombieType {
    Normal,
    Conehead,
}


#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ZombieState {
    Walking,
    Attacking,
    Dead,
}

pub struct Zombie {
    pub zombie_type: ZombieType,
    pub x: f32,
    pub y: f32,
    pub health: i32,
    pub state: ZombieState,
    pub speed: f32,
    pub attack_damage: i32,
    pub attack_cooldown: f32,
    pub last_attack_time: f64,

    pub slow_timer: f32,
    pub speed_multiplier: f32,
}

impl Zombie {
    pub fn new(y: f32, zombie_type: ZombieType) -> Self {

        let (health, speed) = match zombie_type {
            ZombieType::Normal => (100, 20.0),
            ZombieType::Conehead => (200, 20.0), // stronger health, same speed
        };

        Self {
            zombie_type,
            x: SCREEN_WIDTH,
            y,
            health,
            state: ZombieState::Walking,
            speed,
            attack_damage: 20,
            attack_cooldown: 1.0,
            last_attack_time: 0.0,
            slow_timer: 0.0,
            speed_multiplier: 1.0,
        }
    }

    /// Called when hit by slow projectile
    pub fn apply_slow(&mut self, duration: f32) {
        self.slow_timer = duration;
        self.speed_multiplier = 0.5; // move at half speed while slowed
    }

    pub fn update(&mut self, plants: &mut Vec<Plant>) {
        if self.state == ZombieState::Dead {
            return;
        }

        let dt = get_frame_time();

        if self.slow_timer > 0.0 {
            self.slow_timer -= dt;
            if self.slow_timer <= 0.0 {
                self.speed_multiplier = 1.0; // back to normal
            }
        }

        // --- attack logic ---
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
            self.x -= self.speed * self.speed_multiplier * dt;
        }

        // --- death check ---
        if self.health <= 0 {
            self.state = ZombieState::Dead;
        }
    }

    pub fn draw(&self) {
        if self.state == ZombieState::Dead {
            return;
        }

        let base_color = match self.state {
            ZombieState::Walking => GREEN,
            ZombieState::Attacking => RED,
            ZombieState::Dead => DARKGRAY,
        };

        let final_color = if self.slow_timer > 0.0 {
            BLUE
        } else {
            base_color
        };

        // zombie body
        draw_rectangle(self.x - 20.0, self.y - 40.0, 40.0, 80.0, final_color);

        // cone on head
        if self.zombie_type == ZombieType::Conehead {
            draw_triangle(
                vec2(self.x, self.y - 60.0),
                vec2(self.x - 20.0, self.y - 40.0),
                vec2(self.x + 20.0, self.y - 40.0),
                ORANGE,
            );
        }
    }
}
