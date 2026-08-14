use anyhow::Result;
use renderer_vulkan::VulkanRenderer;

use super::{
    CameraSystem, CommandContext, CommandSystem, InputCommand, InputSystem, RenderSystem,
    RotatorSystem, UpdateContext, UpdateSystem,
};

use crate::{Input, Registry};

pub struct Scheduler {
    pub command_system: CommandSystem,
    pub input_system: InputSystem,
    pub render_system: RenderSystem,
    update_systems: Vec<Box<dyn UpdateSystem>>,
}

impl Scheduler {
    pub fn with_input_system(input_system: InputSystem) -> Self {
        Self {
            input_system,
            ..Default::default()
        }
    }

    pub fn new(
        command_system: CommandSystem,
        input_system: InputSystem,
        render_system: RenderSystem,
        update_systems: Vec<Box<dyn UpdateSystem>>,
    ) -> Self {
        Self {
            command_system,
            input_system,
            render_system,
            update_systems,
        }
    }

    pub fn add_update_system(&mut self, update_system: Box<dyn UpdateSystem>) {
        self.update_systems.push(update_system);
    }

    pub fn run_input_stage(&self, input: &Input) -> Vec<InputCommand> {
        self.input_system.update(input)
    }

    pub fn run_command_stage(&self, context: &mut CommandContext<'_>) -> Result<()> {
        self.command_system.update(context)
    }

    pub fn run_update_stage(&mut self, context: &mut UpdateContext<'_>) -> Result<()> {
        for system in &mut self.update_systems {
            system.update(context)?
        }
        Ok(())
    }

    pub fn run_render_stage(
        &mut self,
        registry: &mut Registry,
        renderer: &mut VulkanRenderer,
    ) -> Result<()> {
        self.render_system.update(registry, renderer)
    }
}

impl Default for Scheduler {
    fn default() -> Self {
        Self {
            command_system: CommandSystem,
            input_system: InputSystem::new(),
            render_system: RenderSystem,
            update_systems: vec![Box::new(RotatorSystem), Box::new(CameraSystem)],
        }
    }
}
