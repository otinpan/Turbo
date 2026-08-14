use anyhow::Result;
use turbo_math::Transform;

use super::{UpdateContext, UpdateSystem};
use crate::Rotator;

#[derive(Clone, Debug)]
pub struct RotatorSystem;

impl UpdateSystem for RotatorSystem {
    fn update(&mut self, context: &mut UpdateContext<'_>) -> Result<()> {
        for (_, transform, rotator) in context.registry.query2_mut::<Transform, Rotator>() {
            transform.rotate(rotator.speed * context.delta_time);
        }

        Ok(())
    }
}
