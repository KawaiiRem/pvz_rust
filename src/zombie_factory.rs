use crate::zombie::{basic_zombie::BasicZombie, buckethead_zombie::BucketheadZombie, conehead_zombie::ConeheadZombie, zombie::Zombie};

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum ZombieType {
    Basic,
    Conehead,
    Buckethead,
}

pub fn create_zombie(zombie_type: ZombieType, y: f32) -> Box<dyn Zombie> {
    match zombie_type {
        ZombieType::Basic => Box::new(BasicZombie::new(y)),
        ZombieType::Conehead => Box::new(ConeheadZombie::new(y)),
        ZombieType::Buckethead => Box::new(BucketheadZombie::new(y)), 
    }
}
