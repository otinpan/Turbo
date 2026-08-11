#![allow(
    dead_code,
    unsafe_op_in_unsafe_fn,
    unused_variables,
    clippy::too_many_arguments,
    clippy::unnecessary_wraps
)]

mod rotator_system;
mod camera_system;
mod render_system;

pub use camera_system::CameraSystem;
pub use rotator_system::RotatorSystem;
pub use render_system::RenderSystem;
