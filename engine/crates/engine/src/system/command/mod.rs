#![allow(
    dead_code,
    unsafe_op_in_unsafe_fn,
    unused_variables,
    clippy::too_many_arguments,
    clippy::unnecessary_wraps
)]

mod command_system;
mod despawn_last_command;
mod spawn_primitive_command;
mod spawn_viking_room_command;
mod update_primitive_meshes_command;

pub use command_system::{Command, CommandContext, CommandSystem};
pub use despawn_last_command::DespawnLastCommand;
pub use spawn_primitive_command::SpawnPrimitiveCommand;
pub use spawn_viking_room_command::SpawnVikingRoomCommand;
pub use update_primitive_meshes_command::UpdatePrimitiveMeshesCommand;
