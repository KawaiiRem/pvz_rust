use crate::projectile::{
    instakill_explode::InstakillExplode,
    normal_pea::NormalPea,
    projectile::{Instakill, Projectile},
    slow_pea::SlowPea,
};

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum ProjectileKind {
    Normal,
    Slow,
    Instakill { radius: f32, tier: Instakill },
}

pub struct ProjectileFactory;

impl ProjectileFactory {
    pub fn create(kind: ProjectileKind, x: f32, y: f32) -> Box<dyn Projectile> {
        match kind {
            ProjectileKind::Normal => Box::new(NormalPea::new(x, y)),
            ProjectileKind::Slow => Box::new(SlowPea::new(x, y)),
            ProjectileKind::Instakill { radius, tier } => {
                Box::new(InstakillExplode::new(x, y, radius, tier))
            }
        }
    }
}
