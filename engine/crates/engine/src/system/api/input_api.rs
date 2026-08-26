use cgmath::{Vector2};
use winit::event::{MouseButton};
use winit::keyboard::{KeyCode};
use crate::{Input};

type Vec2=Vector2<f32>;

pub trait InputApi {
    fn input(&self) -> &Input;

    fn mouse_position(&self) -> Vec2 {
        self.input().mouse_position()
    }

    fn mouse_delta(&self) -> Vec2 {
        self.input().mouse_delta()
    }

    fn window_size(&self) -> Vec2 {
        self.input().window_size()
    }

    fn key_down(&self, key: KeyCode) -> bool {
        self.input().key_down(key)
    }

    fn key_pressed(&self, key: KeyCode) -> bool {
        self.input().key_pressed(key)
    }

    fn key_released(&self, key: KeyCode) -> bool {
        self.input().key_released(key)
    }

    fn mouse_button_down(&self, button: MouseButton) -> bool {
        self.input().mouse_button_down(button)
    }
}