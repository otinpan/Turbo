use cgmath::{Vector2, vec2};
use std::collections::HashSet;
use winit::event::{ElementState, MouseButton, WindowEvent};
use winit::keyboard::{KeyCode, PhysicalKey};

pub type Vec2 = Vector2<f32>;

#[derive(Clone, Debug)]
pub struct Input {
    keys_down: HashSet<KeyCode>,
    keys_pressed: HashSet<KeyCode>,
    keys_released: HashSet<KeyCode>,
    mouse_position: Vec2,
    mouse_buttons_down: HashSet<MouseButton>,
    mouse_buttons_pressed: HashSet<MouseButton>,
    mouse_buttons_released: HashSet<MouseButton>,
}

impl Input {
    pub fn clear_transitions(&mut self) {
        self.keys_pressed.clear();
        self.keys_released.clear();
        self.mouse_buttons_pressed.clear();
        self.mouse_buttons_released.clear();
    }

    pub fn handle_event(&mut self, event: &WindowEvent) {
        match event {
            WindowEvent::KeyboardInput { event, .. } => {
                let PhysicalKey::Code(key) = event.physical_key else {
                    return;
                };

                match event.state {
                    ElementState::Pressed => {
                        self.press_key(key);
                    }
                    ElementState::Released => {
                        self.release_key(key);
                    }
                }
            }
            WindowEvent::CursorMoved { position, .. } => {
                self.set_mouse_position(vec2(position.x as f32, position.y as f32));
            }
            WindowEvent::MouseInput { state, button, .. } => match state {
                ElementState::Pressed => {
                    self.press_mouse_button(*button);
                }
                ElementState::Released => {
                    self.release_mouse_button(*button);
                }
            },
            _ => {}
        }
    }

    pub fn press_key(&mut self, key: KeyCode) {
        if self.keys_down.insert(key) {
            self.keys_pressed.insert(key);
        }
    }

    pub fn release_key(&mut self, key: KeyCode) {
        if self.keys_down.remove(&key) {
            self.keys_released.insert(key);
        }
    }

    pub fn key_down(&self, key: KeyCode) -> bool {
        self.keys_down.contains(&key)
    }

    pub fn key_pressed(&self, key: KeyCode) -> bool {
        self.keys_pressed.contains(&key)
    }

    pub fn key_released(&self, key: KeyCode) -> bool {
        self.keys_released.contains(&key)
    }

    pub fn mouse_position(&self) -> Vec2 {
        self.mouse_position
    }

    pub fn set_mouse_position(&mut self, position: Vec2) {
        self.mouse_position = position;
    }

    pub fn press_mouse_button(&mut self, button: MouseButton) {
        if self.mouse_buttons_down.insert(button) {
            self.mouse_buttons_pressed.insert(button);
        }
    }

    pub fn release_mouse_button(&mut self, button: MouseButton) {
        if self.mouse_buttons_down.remove(&button) {
            self.mouse_buttons_released.insert(button);
        }
    }

    pub fn mouse_button_down(&self, button: MouseButton) -> bool {
        self.mouse_buttons_down.contains(&button)
    }

    pub fn mouse_button_pressed(&self, button: MouseButton) -> bool {
        self.mouse_buttons_pressed.contains(&button)
    }

    pub fn mouse_button_released(&self, button: MouseButton) -> bool {
        self.mouse_buttons_released.contains(&button)
    }
}

impl Default for Input {
    fn default() -> Self {
        Self {
            keys_down: HashSet::new(),
            keys_pressed: HashSet::new(),
            keys_released: HashSet::new(),
            mouse_position: vec2(0.0, 0.0),
            mouse_buttons_down: HashSet::new(),
            mouse_buttons_pressed: HashSet::new(),
            mouse_buttons_released: HashSet::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn key_press_sets_down_and_pressed() {
        let mut input = Input::default();

        input.press_key(KeyCode::Space);

        assert!(input.key_down(KeyCode::Space));
        assert!(input.key_pressed(KeyCode::Space));
        assert!(!input.key_released(KeyCode::Space));
    }

    #[test]
    fn repeated_key_press_does_not_repeat_pressed_transition() {
        let mut input = Input::default();

        input.press_key(KeyCode::Space);
        input.clear_transitions();
        input.press_key(KeyCode::Space);

        assert!(input.key_down(KeyCode::Space));
        assert!(!input.key_pressed(KeyCode::Space));
    }

    #[test]
    fn key_release_sets_released_and_clears_down() {
        let mut input = Input::default();

        input.press_key(KeyCode::Space);
        input.clear_transitions();
        input.release_key(KeyCode::Space);

        assert!(!input.key_down(KeyCode::Space));
        assert!(!input.key_pressed(KeyCode::Space));
        assert!(input.key_released(KeyCode::Space));
    }

    #[test]
    fn clear_transitions_keeps_down_state() {
        let mut input = Input::default();

        input.press_key(KeyCode::Space);
        input.press_mouse_button(MouseButton::Left);
        input.clear_transitions();

        assert!(input.key_down(KeyCode::Space));
        assert!(!input.key_pressed(KeyCode::Space));
        assert!(input.mouse_button_down(MouseButton::Left));
        assert!(!input.mouse_button_pressed(MouseButton::Left));
    }

    #[test]
    fn mouse_position_is_updated() {
        let mut input = Input::default();

        input.set_mouse_position(vec2(12.0, 34.0));

        assert_eq!(input.mouse_position(), vec2(12.0, 34.0));
    }
}
