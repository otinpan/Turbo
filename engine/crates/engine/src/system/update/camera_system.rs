use anyhow::Result;
use cgmath::{InnerSpace, vec3};
use turbo_math::Transform;
use winit::event::MouseButton;
use winit::keyboard::KeyCode;

use super::{UpdateContext, UpdateSystem};
use crate::{Camera, EntityApi, InputApi};

#[derive(Clone, Debug)]
pub struct CameraSystem;

impl CameraSystem {
    fn update_camera(&mut self, context: &mut UpdateContext<'_>, delta_time: f32) -> Result<()> {
        let move_speed = 3.0;
        let mouse_sensitivity = 0.003;
        let max_pitch = std::f32::consts::FRAC_PI_2 - 0.01;
        let mouse_delta = context.mouse_delta();
        let right_mouse_down = context.mouse_button_down(MouseButton::Right);
        let move_forward = context.key_down(KeyCode::KeyW);
        let move_backward = context.key_down(KeyCode::KeyS);
        let move_left = context.key_down(KeyCode::KeyA);
        let move_right = context.key_down(KeyCode::KeyD);
        let move_up = context.key_down(KeyCode::ArrowUp);
        let move_down = context.key_down(KeyCode::ArrowDown);

        if let Some((_, transform, camera)) = context.query2_mut_mut::<Transform, Camera>().next() {
            if right_mouse_down {
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

            if move_forward {
                transform.position += direction * move_speed * delta_time;
            }
            if move_backward {
                transform.position -= direction * move_speed * delta_time;
            }
            if move_left {
                transform.position += left * move_speed * delta_time;
            }
            if move_right {
                transform.position -= left * move_speed * delta_time;
            }
            if move_up {
                transform.position += up * move_speed * delta_time;
            }
            if move_down {
                transform.position -= up * move_speed * delta_time;
            }

            camera.target = transform.position + direction;
        }

        Ok(())
    }
}

impl UpdateSystem for CameraSystem {
    fn update(&mut self, context: &mut UpdateContext<'_>) -> Result<()> {
        let delta_time = context.delta_seconds();

        self.update_camera(context, delta_time)
    }
}
