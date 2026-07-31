#![allow(
    dead_code,
    unsafe_op_in_unsafe_fn,
    unused_variables,
    clippy::too_many_arguments,
    clippy::unnecessary_wraps
)]

mod app;
mod component;
mod input;
mod primitive;
mod time;
mod world;

pub use app::App;
pub use component::{CameraComponent, Material, MeshHandle, MeshRenderer};
pub use input::Input;
pub use time::Time;
pub use world::{EntityId, World, WorldObject};
