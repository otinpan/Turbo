use anyhow::Result;

use crate::World;

#[derive(Clone, Debug)]
pub struct RotatorSystem;

impl RotatorSystem {
    pub fn update(&mut self, world: &mut World, delta_time: f32) -> Result<()> {
        for entity in world.entities().to_vec() {
            let Some(rotator) = world.rotator(entity).cloned() else {
                continue;
            };

            let Some(transform) = world.transform_mut(entity) else {
                continue;
            };

            transform.rotate(rotator.speed * delta_time);
        }

        Ok(())
    }
}
