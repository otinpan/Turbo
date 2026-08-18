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

pub use command::{
    Command, CommandContext, CommandSystem, DebugMonitor, DespawnLastCommand,
    SpawnPrimitiveCommand, SpawnVikingRoomCommand, UpdatePrimitiveMeshesCommand,
};
pub use input_system::{CommandQueue, CommandRef, InputSystem, InputTrigger, KeyBinding};
pub use render_command::{RenderCommand, RenderCommandQueue};
pub use render_system::{RenderContext, RenderSystem};
pub use scheduler::Scheduler;
pub use update::{CameraSystem, RotatorSystem, ScheduledUpdateSystem, UpdateContext, UpdateSystem};
