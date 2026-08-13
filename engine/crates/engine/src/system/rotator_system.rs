use anyhow::Result;
use turbo_math::Transform;

use crate::{Registry, Rotator};

#[derive(Clone, Debug)]
pub struct RotatorSystem;

impl RotatorSystem {
    pub fn update(&mut self, registry: &mut Registry, delta_time: f32) -> Result<()> {
        for (_, transform, rotator) in registry.query2_mut::<Transform, Rotator>() {
            transform.rotate(rotator.speed * delta_time);
        }

        Ok(())
    }
}
