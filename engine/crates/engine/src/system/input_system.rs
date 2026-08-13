use renderer_vulkan::PipelineKey;
use winit::keyboard::KeyCode;

use crate::Input;
use crate::primitive::PrimitiveType;

pub struct InputSystem {
    pub key_bindings: Vec<KeyBinding>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum InputCommand {
    DespawnLast,
    SpawnVikingRoom,
    SpawnPrimitive {
        primitive_type: PrimitiveType,
        pipeline_key: PipelineKey,
        texture_name: Option<&'static str>,
    },
    UpdatePrimitiveMeshes,
}

#[derive(Clone, Debug, PartialEq)]
pub struct KeyBinding {
    pub key: KeyCode,
    pub trigger: InputTrigger,
    pub command: InputCommand,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum InputTrigger {
    Pressed,
    Down,
    Released,
}

impl InputSystem {
    pub fn new() -> Self {
        Self {
            key_bindings: Vec::new(),
        }
    }

    pub fn reset(&mut self) {
        self.key_bindings.clear();
    }

    pub fn bind(mut self, key: KeyCode, trigger: InputTrigger, command: InputCommand) -> Self {
        let binding = KeyBinding {
            key,
            trigger,
            command,
        };

        if !self.key_bindings.contains(&binding) {
            self.key_bindings.push(binding);
        }

        self
    }

    pub fn update(&self, input: &Input) -> Vec<InputCommand> {
        let mut commands = Vec::new();

        for binding in &self.key_bindings {
            let triggered = match binding.trigger {
                InputTrigger::Pressed => input.key_pressed(binding.key),
                InputTrigger::Down => input.key_down(binding.key),
                InputTrigger::Released => input.key_released(binding.key),
            };

            if triggered {
                commands.push(binding.command.clone());
            }
        }

        commands
    }
}

impl Default for InputSystem {
    fn default() -> Self {
        Self::new()
    }
}
