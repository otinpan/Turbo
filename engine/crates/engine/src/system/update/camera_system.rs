use anyhow::Result;
use cgmath::{InnerSpace, vec3};
use turbo_math::Transform;
use winit::event::MouseButton;
use winit::keyboard::KeyCode;

use super::{UpdateContext, UpdateSystem};
use crate::{Camera, EntityApi, Input};
use crate::{InputApi};

#[derive(Clone, Debug)]
pub struct CameraSystem;

impl CameraSystem {
    fn update_camera(
        &mut self,
        context: &mut UpdateContext<'_>,
        input: &Input,
        delta_time: f32,
    ) -> Result<()> {
        let move_speed = 3.0;
        let mouse_sensitivity = 0.003;
        let max_pitch = std::f32::consts::FRAC_PI_2 - 0.01;

        if let Some((_, transform, camera)) = context.query2_mut_mut::<Transform, Camera>().next() {
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
        }

        Ok(())
    }
}

impl UpdateSystem for CameraSystem {
    fn update(&mut self, context: &mut UpdateContext<'_>) -> Result<()> {
        let input = context.input().clone();
        let delta_time = context.delta_seconds();

        self.update_camera(context, &input, delta_time)
    }
}
