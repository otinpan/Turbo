use anyhow::Result;
use renderer_vulkan::VulkanRenderer;

use super::{
    CameraSystem, CommandContext, CommandSystem, InputCommand, InputSystem, RenderSystem,
    RotatorSystem,
};

use crate::{Input, Registry};

#[derive(Clone, Debug)]
pub struct Scheduler {
    pub command_system: CommandSystem,
    pub input_system: InputSystem,
    pub camera_system: CameraSystem,
    pub render_system: RenderSystem,
    pub rotator_system: RotatorSystem,
}

impl Scheduler {
    pub fn new(
        command_system: CommandSystem,
        input_system: InputSystem,
        camera_system: CameraSystem,
        render_system: RenderSystem,
        rotator_system: RotatorSystem,
    ) -> Self {
        Self {
            command_system,
            input_system,
            camera_system,
            render_system,
            rotator_system,
        }
    }

    pub fn run_input_stage(&self, input: &Input) -> Vec<InputCommand> {
        self.input_system.update(input)
    }

    pub fn run_command_stage(&self, context: &mut CommandContext<'_>) -> Result<()> {
        self.command_system.update(context)
    }

    pub fn run_update_stage(
        &mut self,
        registry: &mut Registry,
        input: &Input,
        delta_time: f32,
    ) -> Result<()> {
        self.rotator_system.update(registry, delta_time)?;
        self.camera_system.update(registry, input, delta_time)?;
        Ok(())
    }

    pub fn run_render_stage(
        &mut self,
        registry: &mut Registry,
        renderer: &mut VulkanRenderer,
    ) -> Result<()>{
        self.render_system.update(registry,renderer)
    }

}

impl Default for Scheduler {
    fn default() -> Self {
        Self {
            command_system: CommandSystem,
            input_system: InputSystem::new(),
            camera_system: CameraSystem,
            render_system: RenderSystem,
            rotator_system: RotatorSystem,
        }
    }
}
