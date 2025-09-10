use crate::plant::{peashooter::Peashooter, plant::Plant, slow_peashooter::SlowPeashooter, sunflower::Sunflower};


#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum PlantType {
    Peashooter,
    Sunflower,
    SlowPeashooter,
}

impl PlantType {
    pub fn cost(&self) -> i32 {
        match self {
            PlantType::Peashooter => 100,
            PlantType::Sunflower => 50,
            PlantType::SlowPeashooter => 125,
        }
    }

    pub fn cooldown_time(&self) -> f32 {
        match self {
            PlantType::Peashooter => 5.0,
            PlantType::Sunflower => 7.5,
            PlantType::SlowPeashooter => 7.5,
        }
    }
}

pub fn create_plant(plant_type: PlantType, x: f32, y: f32) -> Box<dyn Plant> {
    match plant_type {
        PlantType::Peashooter => Box::new(Peashooter::new(x, y)),
        PlantType::SlowPeashooter => Box::new(SlowPeashooter::new(x, y)),
        PlantType::Sunflower => Box::new(Sunflower::new(x, y)),
    }
}