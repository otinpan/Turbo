#![allow(
    dead_code,
    unsafe_op_in_unsafe_fn,
    unused_variables,
    clippy::too_many_arguments,
    clippy::unnecessary_wraps
)]

mod app;
mod component;
mod ecs;
mod input;
mod primitive;
mod time;
mod world;
mod system;

pub use app::App;
pub use component::{Camera, CameraComponent, Material, MeshRenderer, Rotator, Visibility};
pub use ecs::{ComponentPool, EntityId};
pub use input::Input;
pub use time::Time;
pub use world::World;
pub use system::{CameraSystem, RotatorSystem, RenderSystem};
