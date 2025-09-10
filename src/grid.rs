use crate::constants::*;
use macroquad::prelude::*;

#[derive(Clone, Copy, Debug)]
pub struct Tile {
    pub row: usize,
    pub col: usize,
    pub x: f32,
    pub y: f32,
}

pub struct Grid {
    pub tiles: Vec<Tile>,
}

impl Grid {
    pub fn new() -> Self {
        let mut tiles = Vec::new();
        for row in 0..ROWS {
            for col in 0..COLS {
                let x = col as f32 * TILE_SIZE;
                let y = row as f32 * TILE_SIZE + UI_BAR_HEIGHT;
                tiles.push(Tile { row, col, x, y });
            }
        }
        Grid { tiles }
    }

    pub fn draw(&self) {
        for tile in &self.tiles {
            draw_rectangle_lines(tile.x, tile.y, TILE_SIZE, TILE_SIZE, 2.0, DARKGREEN);
        }
    }

    pub fn get_tile_at(&self, mouse: Vec2) -> Option<Tile> {
        for tile in &self.tiles {
            if mouse.x >= tile.x
                && mouse.x < tile.x + TILE_SIZE
                && mouse.y >= tile.y
                && mouse.y < tile.y + TILE_SIZE
            {
                return Some(*tile);
            }
        }
        None
    }
}
