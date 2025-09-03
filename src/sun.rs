use macroquad::prelude::*;
use crate::constants::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SunSource {
    Natural,
    Plant,
}

#[derive(Debug)]
pub struct Sun {
    pub x: f32,
    pub y: f32,
    pub target_y: f32,   // where the sun will stop
    pub speed: f32,
    pub collected: bool,
    pub value: i32,
    pub source: SunSource,
}

impl Sun {
    /// Spawn a sun produced by a plant
    pub fn from_plant(x: f32, y: f32) -> Self {
        let offset_x = if rand::gen_range(0, 2) == 0 { -20.0 } else { 20.0 };
        let target_y = y + TILE_SIZE / 2.0; // drop just below the plant
        Self {
            x: x + offset_x,
            y,
            target_y,
            speed: 40.0,
            collected: false,
            value: SUN_VALUE,
            source: SunSource::Plant,
        }
    }

    /// Spawn a natural sun from the sky
    pub fn natural() -> Self {
        let x = rand::gen_range(50.0, SCREEN_WIDTH - 50.0);
        let grid_bottom = ROWS as f32 * TILE_SIZE + UI_BAR_HEIGHT;
        let target_y = rand::gen_range(200.0, grid_bottom - 50.0);

        Self {
            x,
            y: 0.0,
            target_y,
            speed: 60.0,
            collected: false,
            value: SUN_VALUE,
            source: SunSource::Natural,
        }
    }

    pub fn update(&mut self, dt: f32) {
        if !self.collected {
            if self.y < self.target_y {
                self.y += self.speed * dt;
                if self.y > self.target_y {
                    self.y = self.target_y;
                }
            }
        }
    }

    pub fn draw(&self) {
        if !self.collected {
            draw_circle(self.x, self.y, 15.0, YELLOW);
            draw_circle(self.x, self.y, 10.0, ORANGE);
        }
    }

    pub fn is_hovered(&mut self, mouse_x: f32, mouse_y: f32) -> bool {
        if !self.collected {
            let dx = mouse_x - self.x;
            let dy = mouse_y - self.y;
            if (dx * dx + dy * dy).sqrt() < 20.0 {
                self.collected = true;
                return true;
            }
        }
        false
    }
}
