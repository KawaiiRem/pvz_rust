use std::clone;

use macroquad::prelude::*;

use crate::constants::*;
use crate::plant::plant::Plant;
use crate::projectile::projectile::Instakill;
use crate::zombie::zombie::{Zombie, ZombieState};

pub struct PoleZombie {
    pub x: f32,
    pub y: f32,
    pub base_y: f32, // fixed lane row
    pub health: i32,
    pub speed: f32,
    pub state: ZombieState,
    pub attack_damage: i32,
    pub attack_cooldown: f32,
    pub last_attack_time: f64,
    pub slow_timer: f32,
    pub speed_multiplier: f32,
    pub has_pole: bool,
    pub jump_start_x: f32,
    pub jump_target_x: f32,
    pub jump_progress: f32,
    pub pole_x: f32,
}

impl PoleZombie {
    pub fn new(y: f32) -> Self {
        Self {
            x: SCREEN_WIDTH,
            y,
            base_y: y,
            health: 100,
            speed: 30.0,
            state: ZombieState::Walking,
            attack_damage: 20,
            attack_cooldown: 1.0,
            last_attack_time: 0.0,
            slow_timer: 0.0,
            speed_multiplier: 1.0,
            has_pole: true,
            jump_start_x: 0.0,
            jump_target_x: 0.0,
            jump_progress: 0.0,
            pole_x: SCREEN_WIDTH + 15.0,
        }
    }
}

impl Zombie for PoleZombie {
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
        matches!(self.state, ZombieState::Dead)
    }

    fn is_instakill(&mut self, _tier: Instakill) {
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

        match self.state {
            ZombieState::Jumping => {
                self.jump_progress += dt * 2.0; // jump speed
                if self.jump_progress >= 1.0 {
                    self.x = self.jump_target_x;
                    self.y = self.base_y;
                    self.state = ZombieState::Walking;
                    self.has_pole = false; // pole disappears after landing
                    self.speed = 20.0;
                } else {
                    let t = self.jump_progress;
                    let height = (1.0 - (2.0 * t - 1.0).powi(2)) * 60.0;
                    self.x = self.jump_start_x + (self.jump_target_x - self.jump_start_x) * t;
                    self.y = self.base_y - height;
                }
            }
            _ => {
                // always reset to base lane when not jumping
                self.y = self.base_y;

                // check for plant collision
                let mut target: Option<&mut Box<dyn Plant>> = None;
                for plant in plants.iter_mut() {
                    if (plant.y() - self.y).abs() < 40.0 && (plant.x() - self.x).abs() < 40.0 {
                        target = Some(plant);
                        break;
                    }
                }

                if let Some(plant) = target {
                    if self.has_pole {
                        // initiate jump
                        self.state = ZombieState::Jumping;
                        self.jump_start_x = self.x;

                        if plant.is_tall() {
                            self.jump_target_x = self.x;
                        } else {
                            self.jump_target_x = self.x - 80.0;
                        }

                        self.jump_progress = 0.0;
                        self.pole_x = self.x - 30.0;
                    } else {
                        // attack
                        self.state = ZombieState::Attacking;
                        let now = get_time();
                        if now - self.last_attack_time >= self.attack_cooldown as f64 {
                            self.last_attack_time = now;
                            plant.take_damage(self.attack_damage);
                        }
                    }
                } else {
                    // walk
                    self.state = ZombieState::Walking;
                    self.x -= self.speed * self.speed_multiplier * dt;
                    self.pole_x = self.x + 15.0;
                }
            }
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

        // zombie body
        draw_rectangle(self.x - 20.0, self.y - 40.0, 40.0, 80.0, color);

        // pole sticks in the ground (not jumping with zombie)
        if self.has_pole {
            draw_rectangle(self.pole_x, self.base_y - 60.0, 5.0, 100.0, BROWN);
        }

        // shadow if jumping
        if self.state == ZombieState::Jumping {
            draw_ellipse(
                self.x,
                self.base_y + 45.0,
                25.0,
                10.0,
                0.0,
                Color::new(0.0, 0.0, 0.0, 0.3),
            );
        }
    }
}
