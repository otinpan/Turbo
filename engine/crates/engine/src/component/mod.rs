#![allow(
    dead_code,
    unsafe_op_in_unsafe_fn,
    unused_variables,
    clippy::too_many_arguments,
    clippy::unnecessary_wraps
)]

mod camera;
mod material;
mod mesh_renderer;

pub use camera::CameraComponent;
pub use material::Material;
pub use mesh_renderer::{MeshRenderer};
