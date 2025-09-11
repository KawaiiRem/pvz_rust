use macroquad::prelude::*;
use crate::game::Game;
use crate::game_over::GameOver;
use crate::game_state::GameState;
use crate::plant_select::PlantSelect;

mod constants;
mod game;
mod grid;
mod plant;
mod plant_bar;
mod projectile;
mod sun;
mod zombie;
mod factory;
mod plant_select;
mod game_state;
mod game_over;

#[macroquad::main("PVZ Rust")]
async fn main() {
    let mut state = GameState::PlantSelect;
    let mut plant_select = PlantSelect::new();
    let mut game: Option<Game> = None;
    let mut game_over = GameOver::new();

    loop {
        match &mut state {
            GameState::PlantSelect => {
                if let Some(selected_plants) = plant_select.update() {
                    game = Some(Game::new(selected_plants));
                    state = GameState::Playing;
                }
                plant_select.draw();
            }
            GameState::Playing => {
                if let Some(g) = &mut game {
                    g.update();
                    g.draw();

                    if g.is_lost() {
                        state = GameState::GameOver;
                        game_over = GameOver::new();
                    }
                }
            }
            GameState::GameOver => {
                game_over.update();
                game_over.draw();

                if game_over.restart {
                    plant_select = PlantSelect::new();
                    state = GameState::PlantSelect;
                }
            }
        }
        next_frame().await;
    }
}