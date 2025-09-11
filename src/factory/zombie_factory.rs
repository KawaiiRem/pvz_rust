use crate::zombie::{
    basic_zombie::BasicZombie, buckethead_zombie::BucketheadZombie,
    conehead_zombie::ConeheadZombie, zombie::Zombie,
};
use macroquad::rand::{self, ChooseRandom}; // <-- use macroquad RNG + trait for choose()

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Rarity {
    Common,
    Uncommon,
    Rare,
    Epic,
    Boss,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum ZombieType {
    Basic,
    Conehead,
    Buckethead,
    PoleZombie,
}

pub fn create_zombie(zombie_type: ZombieType, y: f32) -> Box<dyn Zombie> {
    match zombie_type {
        ZombieType::Basic => Box::new(BasicZombie::new(y)),
        ZombieType::Conehead => Box::new(ConeheadZombie::new(y)),
        ZombieType::Buckethead => Box::new(BucketheadZombie::new(y)),
        ZombieType::PoleZombie => Box::new(crate::zombie::pole_zombie::PoleZombie::new(y)),
    }
}

fn base_rarity_weights() -> Vec<(Rarity, f32)> {
    vec![
        (Rarity::Common, 0.8),
        (Rarity::Uncommon, 0.155),
        (Rarity::Rare, 0.04),
        (Rarity::Epic, 0.0049),
        (Rarity::Boss, 0.0001),
    ]
}

fn get_scaled_rarity_weights(zombie_count: usize) -> Vec<(Rarity, f32)> {
    let mut weights = base_rarity_weights();
    let difficulty = (zombie_count as f32 / 50.0).min(1.0);

    for (rarity, w) in &mut weights {
        match rarity {
            Rarity::Common => *w *= 1.0 - 0.4 * difficulty,
            Rarity::Uncommon => *w *= 1.0 + 0.3 * difficulty,
            Rarity::Rare => *w *= 1.0 + 0.6 * difficulty,
            Rarity::Epic => *w *= 1.0 + 1.0 * difficulty,
            Rarity::Boss => *w *= 1.0 + 1.5 * difficulty,
        }
    }

    let sum: f32 = weights.iter().map(|(_, w)| w).sum();
    for (_, w) in &mut weights {
        *w /= sum;
    }

    weights
}

fn pick_rarity(zombie_count: usize) -> Rarity {
    let roll: f32 = rand::gen_range(0.0, 1.0);
    let weights = get_scaled_rarity_weights(zombie_count);

    let mut acc = 0.0;
    for (rarity, weight) in weights {
        acc += weight;
        if roll < acc {
            return rarity;
        }
    }
    Rarity::Common
}

fn zombies_by_rarity(rarity: Rarity) -> Vec<ZombieType> {
    match rarity {
        Rarity::Common => vec![ZombieType::Basic],
        Rarity::Uncommon => vec![ZombieType::Conehead, ZombieType::PoleZombie],
        Rarity::Rare => vec![ZombieType::Conehead],
        Rarity::Epic => vec![ZombieType::Buckethead],
        Rarity::Boss => vec![ZombieType::Buckethead], // later: add Boss type
    }
}

pub fn spawn_random_zombie(y: f32, zombie_count: usize) -> Box<dyn Zombie> {
    let rarity = pick_rarity(zombie_count);
    let pool = zombies_by_rarity(rarity);

    // `choose` comes from macroquad::rand::ChooseRandom
    let zombie_type = *pool.choose().unwrap_or(&ZombieType::Basic);
    create_zombie(zombie_type, y)
}
