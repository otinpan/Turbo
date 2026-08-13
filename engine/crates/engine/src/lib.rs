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
mod system;
mod time;
mod world;

pub use app::App;
pub use component::{Camera, CameraComponent, Material, MeshRenderer, Rotator, Visibility};
pub use ecs::{ComponentPool, EntityId, Registry};
pub use input::Input;
pub use system::{
    CameraSystem, InputCommand, InputSystem, InputTrigger, KeyBinding, RenderSystem, RotatorSystem,
    Scheduler,
};
pub use time::Time;
pub use world::World;
