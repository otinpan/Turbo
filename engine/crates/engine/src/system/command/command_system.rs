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
