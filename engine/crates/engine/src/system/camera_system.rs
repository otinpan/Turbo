use anyhow::Result;
use cgmath::{InnerSpace, vec3};
use turbo_math::Transform;
use winit::event::MouseButton;
use winit::keyboard::KeyCode;

use crate::{Camera, Input, Registry};

#[derive(Clone, Debug)]
pub struct CameraSystem;

impl CameraSystem {
    pub fn update(&mut self, registry: &mut Registry, input: &Input, delta_time: f32) -> Result<()> {
        let move_speed = 3.0;
        let mouse_sensitivity = 0.003;
        let max_pitch = std::f32::consts::FRAC_PI_2 - 0.01;

        let Some(camera_entity) = registry.active_camera_entity() else {
            return Ok(());
        };

        for (entity, transform, camera) in registry.query2_mut_mut::<Transform, Camera>() {
            if entity != camera_entity {
                continue;
            }

            let mouse_delta = input.mouse_delta();
            if input.mouse_button_down(MouseButton::Right) {
                camera.yaw -= mouse_delta.x * mouse_sensitivity;
                camera.pitch -= mouse_delta.y * mouse_sensitivity;
                camera.pitch = camera.pitch.clamp(-max_pitch, max_pitch);
            }

            let direction = vec3(
                camera.yaw.cos() * camera.pitch.cos(),
                camera.yaw.sin() * camera.pitch.cos(),
                camera.pitch.sin(),
            )
            .normalize();
            let left = vec3(-direction.y, direction.x, 0.0).normalize();
            let up = vec3(0.0, 0.0, 1.0);

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

            camera.target = transform.position + direction;
            break;
        }

        Ok(())
    }
}
