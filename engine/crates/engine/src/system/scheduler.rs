use anyhow::Result;
use renderer_vulkan::VulkanRenderer;

use super::{CameraSystem, InputCommand, InputSystem, RenderSystem, RotatorSystem};

use crate::{Input, Registry};

#[derive(Clone, Debug)]
pub struct Scheduler {
    pub input_system: InputSystem,
    pub camera_system: CameraSystem,
    pub render_system: RenderSystem,
    pub rotator_system: RotatorSystem,
}

impl Scheduler {
    pub fn new(
        input_system: InputSystem,
        camera_system: CameraSystem,
        render_system: RenderSystem,
        rotator_system: RotatorSystem,
    ) -> Self {
        Self {
            input_system,
            camera_system,
            render_system,
            rotator_system,
        }
    }

    pub fn input_commands(&self, input: &Input) -> Vec<InputCommand> {
        self.input_system.update(input)
    }

    pub fn update(
        &mut self,
        registry: &mut Registry,
        renderer: &mut VulkanRenderer,
        input: &Input,
        delta_time: f32,
    ) -> Result<()> {
        self.rotator_system.update(registry, delta_time)?;
        self.camera_system.update(registry, input, delta_time)?;
        self.render_system.update(registry, renderer)?;
        Ok(())
    }
}

impl Default for Scheduler {
    fn default() -> Self {
        Self {
            input_system: InputSystem::new(),
            camera_system: CameraSystem,
            render_system: RenderSystem,
            rotator_system: RotatorSystem,
        }
    }
}
