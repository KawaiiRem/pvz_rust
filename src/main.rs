mod constants;
mod grid;
mod game;
mod plant_bar;
mod plant;
mod projectile;
mod sun;
mod zombie;

use macroquad::prelude::*;
use game::Game;

#[macroquad::main("PVZ Rust")]
async fn main() {
    let mut game = Game::new();

    loop {
        game.update();
        game.draw();

        next_frame().await;
    }
}
