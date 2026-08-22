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
mod resources;
mod system;
mod time;
mod world;

pub use app::App;
pub use component::{
    Camera, CameraComponent, Component, Material, MeshRenderer, Name, PendingPrimitiveMesh,
    Rotator, Tags, Visibility,
};
pub use ecs::{ComponentPool, EntityId, Registry};
pub use input::Input;
pub use primitive::{PrimitiveMesh, PrimitiveShape, PrimitiveType};
pub use resources::{MeshAsset, MeshAssetId, Resources};
pub use system::{
    CameraSystem, Command, CommandContext, CommandQueue, CommandRef, CommandSystem,
    CreatePrimitiveCommand, DebugMonitor, DespawnLastCommand, InputSystem, InputTrigger,
    KeyBinding, PrimitiveSystem, RenderCommand, RenderCommandQueue, RenderSystem, RotatorSystem,
    Scheduler, SpawnPrimitiveCommand, SpawnVikingRoomCommand, UpdateContext,
    UpdatePrimitiveMeshesCommand, UpdateSystem,
};
pub use time::Time;
pub use world::World;
