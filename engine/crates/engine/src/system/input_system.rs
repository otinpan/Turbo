use std::sync::Arc;
use winit::keyboard::KeyCode;

use super::Command;
use crate::Input;

pub type CommandRef = Arc<dyn Command>;
pub type CommandQueue = Vec<CommandRef>;

#[derive(Clone, Debug)]
pub struct InputSystem {
    pub key_bindings: Vec<KeyBinding>,
}

#[derive(Clone)]
pub struct KeyBinding {
    pub key: KeyCode,
    pub trigger: InputTrigger,
    pub command_id: String,
    pub command: CommandRef,
}

impl std::fmt::Debug for KeyBinding {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("KeyBinding")
            .field("key", &self.key)
            .field("trigger", &self.trigger)
            .field("command_id", &self.command_id)
            .finish()
    }
}

impl PartialEq for KeyBinding {
    fn eq(&self, other: &Self) -> bool {
        self.key == other.key
            && self.trigger == other.trigger
            && self.command_id == other.command_id
    }
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

    pub fn bind_in_place<C>(&mut self, key: KeyCode, trigger: InputTrigger, command: C)
    where
        C: Command + 'static,
    {
        self.bind_ref(key, trigger, Arc::new(command));
    }

    pub fn bind_ref(&mut self, key: KeyCode, trigger: InputTrigger, command: CommandRef) {
        let binding = KeyBinding {
            key,
            trigger,
            command_id: command.id(),
            command,
        };

        if !self.key_bindings.contains(&binding) {
            self.key_bindings.push(binding);
        }
    }

    pub fn update(&self, input: &Input) -> CommandQueue {
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
