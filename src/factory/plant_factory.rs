use crate::plant::{
    cherry_bomb::CherryBomb, peashooter::Peashooter, plant::Plant, potato_mine::PotatoMine,
    slow_peashooter::SlowPeashooter, sunflower::Sunflower, wallnut::Wallnut,
};
use macroquad::prelude::*;
use strum_macros::EnumIter;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, EnumIter)]
pub enum PlantType {
    Sunflower,
    Peashooter,
    SlowPeashooter,
    PotatoMine,
    Wallnut,
    CherryBomb,
}
impl PlantType {
    pub fn description(&self) -> &'static str {
        match self {
            PlantType::Peashooter => "Shoots peas at zombies.",
            PlantType::Sunflower => "Produces sun over time.",
            PlantType::SlowPeashooter => "Shoots peas that slow down zombies.",
            PlantType::PotatoMine => "Explodes when a zombie steps on it when fully grown.",
            PlantType::Wallnut => "A sturdy wall that blocks zombies.",
            PlantType::CherryBomb => "A cherry that explodes and damages all zombies in an area.",
        }
    }

    pub fn cost(&self) -> i32 {
        match self {
            PlantType::Peashooter => 100,
            PlantType::Sunflower => 50,
            PlantType::SlowPeashooter => 125,
            PlantType::PotatoMine => 25,
            PlantType::Wallnut => 50,
            PlantType::CherryBomb => 150,
        }
    }

    pub fn cooldown_time(&self) -> f32 {
        match self {
            PlantType::Peashooter => 5.0,
            PlantType::Sunflower => 7.5,
            PlantType::SlowPeashooter => 7.5,
            PlantType::PotatoMine => 25.0,
            PlantType::Wallnut => 30.0,
            PlantType::CherryBomb => 30.0,
        }
    }

    pub fn draw_preview(&self, x: f32, y: f32) {
        match self {
            PlantType::Peashooter => {
                // body + head (small version)
                draw_circle(x, y, 10.0, GREEN);
                draw_circle(x + 10.0, y, 5.0, DARKGREEN);

                // eyes (forward-facing, serious)
                draw_circle(x - 4.0, y - 3.0, 1.2, BLACK);
                draw_circle(x + 2.0, y - 3.0, 1.2, BLACK);
            }
            PlantType::SlowPeashooter => {
                draw_circle(x, y, 10.0, BLUE);
                draw_circle(x + 10.0, y, 5.0, DARKBLUE);

                // eyes
                draw_circle(x - 4.0, y - 3.0, 1.2, BLACK);
                draw_circle(x + 2.0, y - 3.0, 1.2, BLACK);
            }
            PlantType::Sunflower => {
                draw_circle(x, y, 9.0, YELLOW); // petals
                draw_circle(x, y, 5.0, ORANGE); // center

                // happy eyes
                draw_circle(x - 2.5, y - 2.0, 1.0, BLACK);
                draw_circle(x + 2.5, y - 2.0, 1.0, BLACK);

                // smile
                draw_line(x - 3.0, y + 2.0, x + 3.0, y + 2.0, 1.0, BLACK);
            }
            PlantType::PotatoMine => {
                // body
                draw_circle(x, y, 10.0, ORANGE);
                draw_circle(x, y, 9.0, BROWN);

                // eyes
                draw_circle(x - 3.0, y - 2.0, 1.2, WHITE);
                draw_circle(x + 3.0, y - 2.0, 1.2, WHITE);
                draw_circle(x - 3.0, y - 2.0, 0.6, BLACK);
                draw_circle(x + 3.0, y - 2.0, 0.6, BLACK);

                // fuse
                draw_circle(x + 6.0, y - 4.0, 1.5, RED);
            }
            PlantType::Wallnut => {
                // main oval body (taller than before)
                draw_ellipse(x, y, 8.0, 12.0, 0.0, BROWN);
                draw_ellipse(x, y, 7.0, 11.0, 0.0, DARKBROWN);

                // goofy eyes
                draw_circle(x - 2.5, y - 3.0, 1.2, WHITE);
                draw_circle(x + 2.5, y - 3.0, 1.2, WHITE);
                draw_circle(x - 2.5, y - 3.0, 0.6, BLACK);
                draw_circle(x + 2.5, y - 3.0, 0.6, BLACK);

                // small flat mouth
                draw_line(x - 3.0, y + 4.0, x + 3.0, y + 4.0, 1.0, BLACK);
            }
            PlantType::CherryBomb => {
                // two cherries with stem
                // Left cherry
                draw_circle(x - 6.0, y, 7.0, RED);
                // Right cherry
                draw_circle(x + 6.0, y, 7.0, RED);

                // Cherry shine highlight
                draw_circle(x - 7.5, y - 2.5, 2.0, PINK);
                draw_circle(x + 4.5, y - 2.5, 2.0, PINK);

                // Green stem
                draw_line(x - 2.5, y - 7.0, x + 2.5, y - 11.0, 1.5, DARKGREEN);
            }
        }
    }
}

pub fn create_plant(plant_type: PlantType, x: f32, y: f32) -> Box<dyn Plant> {
    match plant_type {
        PlantType::Peashooter => Box::new(Peashooter::new(x, y)),
        PlantType::SlowPeashooter => Box::new(SlowPeashooter::new(x, y)),
        PlantType::Sunflower => Box::new(Sunflower::new(x, y)),
        PlantType::PotatoMine => Box::new(PotatoMine::new(x, y)),
        PlantType::Wallnut => Box::new(Wallnut::new(x, y)),
        PlantType::CherryBomb => Box::new(CherryBomb::new(x, y)),
    }
}
