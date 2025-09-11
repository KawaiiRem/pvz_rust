use crate::constants::{SCREEN_HEIGHT, SCREEN_WIDTH};
use crate::game::Game;
use crate::game_over::GameOver;
use crate::game_state::GameState;
use crate::plant_select::PlantSelect;
use macroquad::prelude::*;

mod constants;
mod factory;
mod game;
mod game_over;
mod game_state;
mod grid;
mod plant;
mod plant_bar;
mod plant_select;
mod projectile;
mod sun;
mod zombie;

fn window_conf() -> Conf {
    Conf {
        window_title: "PVZ Rust".to_string(),
        window_width: SCREEN_WIDTH as i32,
        window_height: SCREEN_HEIGHT as i32 + 100,
        fullscreen: false,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
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
                        game = None;
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
