mod constants;
mod game;
mod grid;
mod plant;
mod plant_bar;
mod projectile;
mod sun;
mod zombie;
mod plant_factory;
mod zombie_factory;

use game::Game;
use macroquad::prelude::*;

#[macroquad::main("PVZ Rust")]
async fn main() {
    let mut game = Game::new();

    loop {
        game.update();
        game.draw();

        next_frame().await;
    }
}
