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
mod runner;
mod scene;
mod scene_manager;
mod system;
mod time;
mod world;

pub use app::App;
pub use component::{
    Camera, CameraComponent, Component, Material, MeshRenderer, Name, PendingPrimitiveMesh,
    Rotator, SceneId, SceneOwned, Tags, Visibility,
};
pub use ecs::{ComponentPool, EntityId, Registry};
pub use input::Input;
pub use primitive::{PrimitiveMesh, PrimitiveShape, PrimitiveType};
pub use renderer_vulkan::PipelineKey;
pub use resources::{MeshAsset, MeshAssetId, Resources};
pub use runner::{EngineConfig, run};
pub use scene::{Scene, SceneContext};
pub use scene_manager::SceneManager;
pub use system::{
    AssetApi, CameraSystem, Command, CommandContext, CommandQueue, CommandRef, CommandSystem,
    CreatePrimitiveCommand, DebugMonitor, DespawnLastCommand, EntityApi, InputApi, InputSystem,
    InputTrigger, KeyBinding, ObjectApi, RenderCommand, RenderCommandApi, RenderCommandQueue,
    RenderSystem, RotatorSystem, Scheduler, SpawnPrimitiveCommand, SpawnVikingRoomCommand, TimeApi,
    UpdateContext, UpdatePrimitiveMeshesCommand, UpdateSystem,
};
pub use time::Time;
pub use world::World;



pub mod prelude {
    pub use crate::{
        App, EngineConfig, run,
        Scene, SceneContext,
        Command, CommandContext,
        UpdateContext, UpdateSystem,
        CameraSystem,
        AssetApi, EntityApi, InputApi, ObjectApi, RenderCommandApi, TimeApi,
        InputTrigger,
        Component, EntityId,
        Camera, Material, MeshRenderer, Name, Rotator, Tags, Visibility,
        PrimitiveShape, PrimitiveType,
        PipelineKey,
    };

    pub use turbo_math::Transform;
}