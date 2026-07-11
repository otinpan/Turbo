#![allow(
    dead_code,
    unsafe_op_in_unsafe_fn,
    unused_variables,
    clippy::too_many_arguments,
    clippy::unnecessary_wraps
)]

mod camera;
mod mesh_renderer;


pub use mesh_renderer::{MeshRenderer,MeshHandle};
pub use camera::{CameraComponent};