use anyhow::Result;
use renderer_vulkan::VulkanRenderer;

use super::{
    Command, CommandContext, CommandQueue, CommandSystem, InputSystem, InputTrigger, RenderSystem,
    ScheduledUpdateSystem, UpdateContext, UpdateSystem,
};

use crate::{Input, Registry};

pub struct Scheduler {
    pub command_system: CommandSystem,
    pub input_system: InputSystem,
    pub render_system: RenderSystem,
    update_systems: Vec<ScheduledUpdateSystem>,
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
        update_systems: Vec<ScheduledUpdateSystem>,
    ) -> Self {
        Self {
            command_system,
            input_system,
            render_system,
            update_systems,
        }
    }

    pub fn add_update_system<S>(&mut self, name: &str, system: S)
    where
        S: UpdateSystem + 'static,
    {
        if self.update_systems.iter().any(|s| s.name == name) {
            return;
        }

        self.update_systems.push(ScheduledUpdateSystem {
            name: name.to_string(),
            system: Box::new(system),
            enabled: true,
        });
    }

    pub fn bind_key<C>(&mut self, key: winit::keyboard::KeyCode, trigger: InputTrigger, command: C)
    where
        C: Command + 'static,
    {
        self.input_system.bind_in_place(key, trigger, command);
    }

    pub fn run_input_stage(&self, input: &Input) -> CommandQueue {
        self.input_system.update(input)
    }

    pub fn run_command_stage(&self, context: &mut CommandContext<'_>) -> Result<()> {
        self.command_system.update(context)
    }

    pub fn run_update_stage(&mut self, context: &mut UpdateContext<'_>) -> Result<()> {
        for scheduled_system in &mut self.update_systems {
            if scheduled_system.enabled {
                scheduled_system.system.update(context)?
            }
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
            update_systems: Vec::new(),
        }
    }
}
