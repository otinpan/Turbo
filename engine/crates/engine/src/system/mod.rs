#![allow(
    dead_code,
    unsafe_op_in_unsafe_fn,
    unused_variables,
    clippy::too_many_arguments,
    clippy::unnecessary_wraps
)]

mod command;
mod input_system;
mod render_command;
mod render_system;
mod scheduler;
mod update;
mod entity_api;
mod object_api;

pub use command::{
    Command, CommandContext, CommandSystem, CreatePrimitiveCommand, DebugMonitor,
    DespawnLastCommand, SpawnPrimitiveCommand, SpawnVikingRoomCommand,
    UpdatePrimitiveMeshesCommand,
};
pub use object_api::{ObjectApi};
pub use entity_api::{EntityApi};
pub use input_system::{CommandQueue, CommandRef, InputSystem, InputTrigger, KeyBinding};
pub use render_command::{RenderCommand, RenderCommandQueue};
pub use render_system::{RenderContext, RenderSystem};
pub use scheduler::Scheduler;
pub use update::{CameraSystem, RotatorSystem, ScheduledUpdateSystem, UpdateContext, UpdateSystem};
