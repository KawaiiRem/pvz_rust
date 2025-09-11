use crate::{constants::*, zombie::zombie::Zombie};
use macroquad::prelude::*;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Instakill{
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

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum ProjectileType {
    Normal,
    Slow,
    Instakill { radius: f32, tier: Instakill },
}

#[derive(Debug)]
pub struct Projectile {
    pub x: f32,
    pub y: f32,
    pub speed: f32,
    pub damage: i32,
    pub active: bool,
    pub projectile_type: ProjectileType,
    pub lifetime: f32,
}

impl Projectile {
    pub fn new(x: f32, y: f32, projectile_type: ProjectileType) -> Self {
        match projectile_type {
            ProjectileType::Normal => Self {
                x,
                y,
                speed: 300.0,
                damage: 20,
                active: true,
                projectile_type,
                lifetime: 0.0,
            },
            ProjectileType::Slow => Self {
                x,
                y,
                speed: 300.0,
                damage: 10,
                active: true,
                projectile_type,
                lifetime: 0.0,
            },
            ProjectileType::Instakill { radius, tier } => Self {
                x,
                y,
                speed: 0.0,
                damage: -1,
                active: true,
                projectile_type: ProjectileType::Instakill { radius: radius, tier: tier },
                lifetime: 0.6,
            },
        }
    }

    pub fn update(&mut self, dt: f32, zombies: &mut Vec<Box<dyn Zombie>>) {
        if !self.active {
            return;
        }

        match &mut self.projectile_type {
            ProjectileType::Normal | ProjectileType::Slow => {
                // move
                self.x += self.speed * dt;

                // out of screen
                if self.x > screen_width() {
                    self.active = false;
                    return;
                }

                // check collisions
                for zombie in zombies.iter_mut() {
                    if zombie.is_dead() {
                        continue;
                    }

                    let same_row = (zombie.y() - self.y).abs() < TILE_HEIGHT / 2.0;
                    let hitbox = (zombie.x() - self.x).abs() < 20.0;

                    if same_row && hitbox {
                        zombie.take_damage(self.damage);

                        if let ProjectileType::Slow = self.projectile_type {
                            zombie.apply_slow(2.5);
                        }

                        self.active = false;
                        break;
                    }
                }
            }

            ProjectileType::Instakill { radius , tier} => {
                self.lifetime -= dt;

                // damage zombies in radius
                for z in zombies.iter_mut() {
                    if z.is_dead() {
                        continue;
                    }
                    let dx = z.x() - self.x;
                    let dy = z.y() - self.y;
                    if (dx * dx + dy * dy).sqrt() <= *radius {
                        z.is_instakill(*tier); 
                    }
                }

                if self.lifetime <= 0.0 {
                    self.active = false;
                }
            }
        }
    }

    pub fn draw(&self) {
        if !self.active {
            return;
        }

        match self.projectile_type {
            ProjectileType::Normal => {
                draw_circle(self.x, self.y, 5.0, DARKGREEN);
            }
            ProjectileType::Slow => {
                draw_circle(self.x, self.y, 6.0, DARKBLUE);
            }
            ProjectileType::Instakill { radius, tier } => {
                let alpha = (self.lifetime * 2.0).min(1.0);
                let color = Color::new(1.0, 0.2, 0.2, alpha);
                draw_circle(self.x, self.y, radius, color);
                draw_circle_lines(self.x, self.y, radius + 10.0, 3.0, ORANGE);
            }
        }
    }
}
