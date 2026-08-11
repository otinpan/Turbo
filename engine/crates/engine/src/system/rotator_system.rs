use anyhow::Result;

use crate::Registry;

#[derive(Clone, Debug)]
pub struct RotatorSystem;

impl RotatorSystem {
    pub fn update(&mut self, registry: &mut Registry, delta_time: f32) -> Result<()> {
        for entity in registry.entities().to_vec() {
            let Some(rotator) = registry.rotator(entity).cloned() else {
                continue;
            };

            let Some(transform) = registry.transform_mut(entity) else {
                continue;
            };

            transform.rotate(rotator.speed * delta_time);
        }

        Ok(())
    }
}
