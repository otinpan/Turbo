#![allow(
    dead_code,
    unsafe_op_in_unsafe_fn,
    unused_variables,
    clippy::too_many_arguments,
    clippy::unnecessary_wraps
)]

mod camera;
mod mesh_renderer;
mod material;

pub use camera::CameraComponent;
pub use mesh_renderer::{MeshHandle, MeshRenderer};
pub use material::{Material};
