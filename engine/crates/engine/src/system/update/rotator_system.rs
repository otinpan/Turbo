use anyhow::Result;
use turbo_math::Transform;

use super::{UpdateContext, UpdateSystem};
use crate::{EntityApi, Rotator};

#[derive(Clone, Debug)]
pub struct RotatorSystem;

impl UpdateSystem for RotatorSystem {
    fn update(&mut self, context: &mut UpdateContext<'_>) -> Result<()> {
        let delta_time = context.delta_seconds();

        for (_, transform, rotator) in context.query2_mut::<Transform, Rotator>() {
            transform.rotate(rotator.speed * delta_time);
        }

        Ok(())
    }
}
