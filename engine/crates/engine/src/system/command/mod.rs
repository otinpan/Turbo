#![allow(
    dead_code,
    unsafe_op_in_unsafe_fn,
    unused_variables,
    clippy::too_many_arguments,
    clippy::unnecessary_wraps
)]

mod despawn_last_command;
mod spawn_primitive_command;
mod spawn_viking_room_command;
mod update_primitive_meshes_command;
mod debug_monitor;

pub use despawn_last_command::DespawnLastCommand;
pub use spawn_primitive_command::SpawnPrimitiveCommand;
pub use spawn_viking_room_command::SpawnVikingRoomCommand;
pub use update_primitive_meshes_command::UpdatePrimitiveMeshesCommand;
pub use debug_monitor::DebugMonitor;

use anyhow::Result;
use cgmath::Vector3;
use renderer_vulkan::VulkanRenderer;

use crate::{Input, Resources, World};

use crate::CommandQueue;

pub type Vec3 = Vector3<f32>;

pub struct CommandContext<'a> {
    pub commands: &'a mut CommandQueue,
    pub world: &'a mut World,
    pub renderer: &'a mut VulkanRenderer,
    pub input: &'a Input,
    pub resources: &'a mut Resources,
    pub positions: &'a [Vec3],
}

pub trait Command {
    fn id(&self) -> String;
    fn execute(&self, context: &mut CommandContext<'_>) -> Result<()>;
}

#[derive(Clone, Debug)]
pub struct CommandSystem;

impl CommandSystem {
    pub fn update(&self, context: &mut CommandContext<'_>) -> Result<()> {
        let commands = context.commands.drain(..).collect::<Vec<_>>();

        for command in commands {
            command.execute(context)?;
        }

        Ok(())
    }
}
