#![allow(
    dead_code,
    unsafe_op_in_unsafe_fn,
    unused_variables,
    clippy::too_many_arguments,
    clippy::unnecessary_wraps
)]

mod camera_system;
mod input_system;
mod render_system;
mod rotator_system;
mod scheduler;

pub use camera_system::CameraSystem;
pub use input_system::{InputCommand, InputSystem, InputTrigger, KeyBinding};
pub use render_system::RenderSystem;
pub use rotator_system::RotatorSystem;
pub use scheduler::Scheduler;
