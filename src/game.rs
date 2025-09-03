use std::cmp::min;

use macroquad::prelude::*;
use crate::grid::Grid;
use crate::plant::{Plant, PlantAction};
use crate::plant_bar::UIBar;
use crate::projectile::Projectile;
use crate::sun::Sun;
use crate::constants::*;
use crate::zombie::{Zombie, ZombieState, ZombieType};

pub struct Game {
    pub grid: Grid,
    pub plant_bar: UIBar,
    pub plants: Vec<Plant>,
    pub projectiles: Vec<Projectile>,
    pub suns: Vec<Sun>,
    pub sun_points: i32,
    pub natural_sun_timer: f32,
    pub next_natural_sun_time: f32,
    pub zombies: Vec<Zombie>,
    pub zombie_timer: f32,
    pub next_zombie_time: f32,
    pub zombie_count: i32,
}

impl Game {
    pub fn new() -> Self {
        Game {
            grid: Grid::new(),
            plant_bar: UIBar::new(),
            plants: Vec::new(),
            projectiles: Vec::new(),
            suns: Vec::new(),
            sun_points: 50, // starting points
            natural_sun_timer: 0.0,
            next_natural_sun_time: rand::gen_range(
                NATURAL_SUN_MIN_INTERVAL,
                NATURAL_SUN_MAX_INTERVAL,
            ),
            zombies: Vec::new(),
            zombie_timer: 0.0,
            next_zombie_time: rand::gen_range(
                NATURAL_ZOMBIE_MIN_SPAWN_INTERVAL, 
                NATURAL_ZOMBIE_MAX_SPAWN_INTERVAL,
            ),
            zombie_count: 0,
        }
    }

    pub fn update(&mut self) {
        let dt = get_frame_time();
        self.plant_bar.update();

        let mouse: Vec2 = mouse_position().into();

        for sun in &mut self.suns {
            if sun.is_hovered(mouse.x, mouse.y) {
                self.sun_points += sun.value;
            }
        }
        // --- place plant logic ---
        if is_mouse_button_pressed(MouseButton::Left) {
            // Then, try to place a plant
            if let Some(plant_type) = self.plant_bar.selected {
                if self.sun_points >= plant_type.cost() {
                    if let Some(tile) = self.grid.get_tile_at(mouse) {
                        let x = tile.x + TILE_SIZE / 2.0;
                        let y = tile.y + TILE_SIZE / 2.0;

                        // check if tile is already occupied
                        let occupied = self.plants.iter().any(|p| {
                            (p.x - x).abs() < TILE_SIZE * 0.5 &&
                            (p.y - y).abs() < TILE_SIZE * 0.5
                        });

                        if !occupied {
                            // find the corresponding slot
                            if let Some(slot) = self.plant_bar.slots.iter_mut()
                                .find(|s| s.plant == plant_type) 
                            {
                                if slot.cooldown <= 0.0 {
                                    // can plant
                                    self.sun_points -= plant_type.cost();
                                    self.plants.push(Plant::new(plant_type, x, y));

                                    // start cooldown
                                    slot.cooldown = plant_type.planting_cooldown();

                                    // clear selection
                                    self.plant_bar.selected = None;
                                    for s in &mut self.plant_bar.slots {
                                        s.selected = false;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        // --- update plants (check zombies) ---
        for plant in &mut self.plants {
            if let Some(action) = plant.update(dt, &self.zombies) {
                match action {
                    PlantAction::Shoot(proj) => {
                        self.projectiles.push(proj);
                    }
                    PlantAction::ProduceSun { x, y } => {
                        self.suns.push(Sun::from_plant(x, y));
                    }
                }
            }
        }

        // --- update projectiles (damage zombies) ---
        for proj in &mut self.projectiles {
            proj.update(dt, &mut self.zombies);
        }
        self.projectiles.retain(|p| p.active);

        // --- update suns ---
        for sun in &mut self.suns {
            sun.update(dt);
        }
        self.suns.retain(|s| !s.collected);

        // Remove dead plants
        self.plants.retain(|p| !p.is_dead());

        // --- update zombies ---
        for zombie in &mut self.zombies {
            zombie.update(&mut self.plants);
        }
        self.zombies.retain(|z| z.state != ZombieState::Dead);

        // --- spawn natural suns ---
        self.natural_sun_timer += dt;
        if self.natural_sun_timer >= self.next_natural_sun_time {
            self.suns.push(Sun::natural());
            self.natural_sun_timer = 0.0;
            self.next_natural_sun_time = rand::gen_range(
                NATURAL_SUN_MIN_INTERVAL,
                NATURAL_SUN_MAX_INTERVAL,
            );
        }

        // --- spawn zombies ---
        self.zombie_timer += dt;
        if self.zombie_timer >= self.next_zombie_time {
            let spawn_amount = min(self.zombie_count / 5 + 1, MAX_ZOMBIE_SPAWN);

            let mut chosen_lanes = Vec::new();

            for _ in 0..rand::gen_range(1,spawn_amount + 1) {
                let mut lane;
                let mut attempts = 0;
                loop {
                    lane = rand::gen_range(0, ROWS);
                    attempts += 1;

                    if !chosen_lanes.contains(&lane) || attempts > ROWS {
                        break;
                    }
                }

                if !chosen_lanes.contains(&lane) {
                    chosen_lanes.push(lane);

                    let y = lane as f32 * TILE_SIZE + TILE_SIZE / 2.0 + UI_BAR_HEIGHT;

                    let z_type = if rand::gen_range(0, 4) == 0 && self.zombie_count > 3 {
                        ZombieType::Conehead
                    } else {
                        ZombieType::Normal
                    };

                    self.zombies.push(Zombie::new(y, z_type));
                    self.zombie_count += 1;
                }
            }

            self.zombie_timer = 0.0;

            self.next_zombie_time = rand::gen_range(
                (NATURAL_ZOMBIE_MIN_SPAWN_INTERVAL - (self.zombie_count as f32 / 50.0)).max(2.0),
                (NATURAL_ZOMBIE_MAX_SPAWN_INTERVAL - (self.zombie_count as f32 / 50.0)).max(4.0),
            );
        }

    }

    pub fn draw(&self) {
        clear_background(SKYBLUE);
        self.grid.draw();
        self.plant_bar.draw(self.sun_points);

        for plant in &self.plants {
            plant.draw();
        }

        for proj in &self.projectiles {
            proj.draw();
        }

        for sun in &self.suns {
            sun.draw();
        }

        for zombie in &self.zombies {
            zombie.draw();
        }

        // tile highlight under mouse
        let mouse = mouse_position().into();
        if let Some(tile) = self.grid.get_tile_at(mouse) {
            draw_rectangle_lines(
                tile.x,
                tile.y,
                TILE_SIZE,
                TILE_SIZE,
                4.0,
                YELLOW,
            );
        }
    }
}
