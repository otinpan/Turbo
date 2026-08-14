#![allow(
    dead_code,
    unsafe_op_in_unsafe_fn,
    unused_variables,
    clippy::too_many_arguments,
    clippy::unnecessary_wraps
)]

mod camera_system;
mod rotator_system;
mod update_system;

pub use camera_system::CameraSystem;
pub use rotator_system::RotatorSystem;
pub use update_system::{UpdateContext, UpdateSystem, ScheduledUpdateSystem};
