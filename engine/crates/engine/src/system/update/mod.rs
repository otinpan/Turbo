#![allow(
    dead_code,
    unsafe_op_in_unsafe_fn,
    unused_variables,
    clippy::too_many_arguments,
    clippy::unnecessary_wraps
)]

mod camera_system;
mod rotator_system;

pub use camera_system::CameraSystem;
pub use rotator_system::RotatorSystem;

use crate::{Input, Registry};
use anyhow::Result;

pub struct UpdateContext<'a> {
    pub registry: &'a mut Registry,
    pub input: &'a Input,
    pub delta_time: f32,
}

pub struct ScheduledUpdateSystem {
    pub name: String,
    pub system: Box<dyn UpdateSystem>,
    pub enabled: bool,
}
pub trait UpdateSystem {
    fn update(&mut self, context: &mut UpdateContext<'_>) -> Result<()>;
}
