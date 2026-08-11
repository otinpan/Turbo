use anyhow::Result;
use cgmath::{InnerSpace, vec3};
use winit::event::MouseButton;
use winit::keyboard::KeyCode;

use crate::{Input, World};

#[derive(Clone, Debug)]
pub struct CameraSystem;

impl CameraSystem {
    pub fn update(&mut self, world: &mut World, input: &Input, delta_time: f32) -> Result<()> {
        let move_speed = 3.0;
        let mouse_sensitivity = 0.003;
        let max_pitch = std::f32::consts::FRAC_PI_2 - 0.01;

        let Some(camera_entity) = world.active_camera_entity() else {
            return Ok(());
        };

        let Some((yaw, pitch)) = world.camera_mut(camera_entity).map(|camera| {
            let mouse_delta = input.mouse_delta();
            if input.mouse_button_down(MouseButton::Right) {
                camera.yaw -= mouse_delta.x * mouse_sensitivity;
                camera.pitch -= mouse_delta.y * mouse_sensitivity;
                camera.pitch = camera.pitch.clamp(-max_pitch, max_pitch);
            }

            (camera.yaw, camera.pitch)
        }) else {
            return Ok(());
        };

        let direction = vec3(
            yaw.cos() * pitch.cos(),
            yaw.sin() * pitch.cos(),
            pitch.sin(),
        )
        .normalize();
        let left = vec3(-direction.y, direction.x, 0.0).normalize();
        let up = vec3(0.0, 0.0, 1.0);

        let Some(transform) = world.transform_mut(camera_entity) else {
            return Ok(());
        };

        if input.key_down(KeyCode::KeyW) {
            transform.position += direction * move_speed * delta_time;
        }
        if input.key_down(KeyCode::KeyS) {
            transform.position -= direction * move_speed * delta_time;
        }
        if input.key_down(KeyCode::KeyA) {
            transform.position += left * move_speed * delta_time;
        }
        if input.key_down(KeyCode::KeyD) {
            transform.position -= left * move_speed * delta_time;
        }
        if input.key_down(KeyCode::ArrowUp) {
            transform.position += up * move_speed * delta_time;
        }
        if input.key_down(KeyCode::ArrowDown) {
            transform.position -= up * move_speed * delta_time;
        }

        let target = transform.position + direction;
        if let Some(camera) = world.camera_mut(camera_entity) {
            camera.target = target;
        }

        Ok(())
    }
}
