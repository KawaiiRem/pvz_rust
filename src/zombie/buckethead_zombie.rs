use macroquad::prelude::*;

use crate::{
    plant::plant::Plant, projectile::Instakill, zombie::{basic_zombie::BasicZombie, zombie::Zombie}
};

pub struct BucketheadZombie {
    inner: BasicZombie,
}

impl BucketheadZombie {
    pub fn new(y: f32) -> Self {
        let mut z = BasicZombie::new(y);
        z.health = 350;
        Self { inner: z }
    }
}

impl Zombie for BucketheadZombie {
    fn x(&self) -> f32 {
        self.inner.x()
    }
    fn y(&self) -> f32 {
        self.inner.y()
    }
    fn health(&self) -> i32 {
        self.inner.health()
    }
    fn take_damage(&mut self, amount: i32) {
        self.inner.take_damage(amount);
    }
    fn apply_slow(&mut self, duration: f32) {
        self.inner.apply_slow(duration);
    }
    fn update(&mut self, plants: &mut Vec<Box<dyn Plant>>) {
        self.inner.update(plants);
    }
    fn is_dead(&self) -> bool {
        self.inner.is_dead()
    }
    fn is_instakill(&mut self, tier: Instakill) {
        self.inner.is_instakill(tier);
    }

    fn draw(&self) {
        self.inner.draw();

        let x = self.inner.x();
        let y = self.inner.y() - 60.0; 
        let bucket_width = 40.0;
        let bucket_height = 30.0;

        draw_rectangle(
            x - bucket_width / 2.0,
            y,
            bucket_width,
            bucket_height,
            GRAY,
        );
    }
}
