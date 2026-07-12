use anyhow::Result;
use cgmath::{InnerSpace, vec3};
use renderer_vulkan::{RenderCamera, RenderItem, VulkanRenderer};
use turbo_math::Transform;
use winit::event::{MouseButton, WindowEvent};
use winit::keyboard::KeyCode;
use winit::window::Window;

use super::CameraComponent;
use super::Input;
use super::MeshHandle;
use super::MeshRenderer;
use super::Time;
use super::World;

pub type Vec3 = cgmath::Vector3<f32>;

pub struct App {
    pub renderer: VulkanRenderer,
    pub world: World,
    pub input: Input,
    pub time: Time,
    mesh: MeshHandle,
    positions: Vec<Vec3>,
}

impl App {
    pub unsafe fn create(window: &Window) -> Result<Self> {
        let mut renderer = VulkanRenderer::create(window)?;
        let mut world = World::default();

        let mesh = MeshHandle(renderer.load_mesh_from_model("assets/models/viking_room.obj")?);
        let positions = vec![
            vec3(0.0, -1.25, 1.0),
            vec3(0.0, 1.25, 1.0),
            vec3(0.0, -1.25, -1.0),
            vec3(0.0, 1.25, -1.0),
        ];

        world.spawn(
            Transform {
                position: vec3(5.0, 0.0, 0.0),
                ..Default::default()
            },
            None,
            Some(CameraComponent {
                target: vec3(0.0, 0.0, 0.0),
                fov_y: 45.0,
                near: 0.1,
                far: 100.0,
                yaw: std::f32::consts::PI,
                pitch: 0.0,
            }),
            vec3(0.0, 0.0, 0.0),
        );

        let mut app = Self {
            renderer,
            world,
            input: Input::default(),
            time: Time::default(),
            mesh,
            positions,
        };
        app.prepare_renderer();

        Ok(app)
    }

    pub fn handle_event(&mut self, event: &WindowEvent) {
        self.input.handle_event(event);
    }

    pub unsafe fn render(&mut self, window: &Window) -> Result<()> {
        self.renderer.render(window)
    }

    pub fn update(&mut self) -> Result<()> {
        self.time.update();

        // TODO: later move to systems/rotator.rs
        self.process_input();
        self.update_world()?;
        self.update_camera();

        self.prepare_renderer();
        self.input.clear_transitions();
        Ok(())
    }

    fn update_world(&mut self) -> Result<()> {
        self.world.update(self.time.delta_seconds())
    }

    fn process_input(&mut self) {
        if self.input.key_pressed(KeyCode::ArrowLeft) {
            let id = self.world.objects().last().map(|object| object.id);

            if let Some(id) = id {
                self.world.despawn(id);
            }
        }
        if self.input.key_pressed(KeyCode::ArrowRight) {
            let index = self
                .world
                .objects()
                .iter()
                .filter(|object| object.mesh_renderer.is_some())
                .count();

            if self.positions.len() > index {
                let id = self.world.spawn(
                    Transform {
                        position: self.positions[index],
                        ..Default::default()
                    },
                    Some(MeshRenderer { mesh: self.mesh }),
                    None,
                    vec3(20.0, 0.0, 0.0),
                );

            }
        }
    }

    // TODO: later move to systems/camera.rs
    fn update_camera(&mut self) {
        let delta = self.time.delta_seconds();
        let move_speed = 3.0;
        let mouse_sensitivity = 0.003;
        let max_pitch = std::f32::consts::FRAC_PI_2 - 0.01;

        let Some(camera_id) = self.world.active_camera().map(|object| object.id) else {
            return;
        };

        let Some(camera_object) = self.world.get_mut(camera_id) else {
            return;
        };

        let Some(camera) = camera_object.camera.as_mut() else {
            return;
        };

        let mouse_delta = self.input.mouse_delta();
        if self.input.mouse_button_down(MouseButton::Right) {
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
        let right = vec3(-direction.y, direction.x, 0.0).normalize();
        let up = vec3(0.0, 0.0, 1.0);

        if self.input.key_down(KeyCode::KeyW) {
            camera_object.transform.position += direction * move_speed * delta;
        }
        if self.input.key_down(KeyCode::KeyS) {
            camera_object.transform.position -= direction * move_speed * delta;
        }
        if self.input.key_down(KeyCode::KeyA) {
            camera_object.transform.position -= right * move_speed * delta;
        }
        if self.input.key_down(KeyCode::KeyD) {
            camera_object.transform.position += right * move_speed * delta;
        }
        if self.input.key_down(KeyCode::ArrowUp) {
            camera_object.transform.position += up * move_speed * delta;
        }
        if self.input.key_down(KeyCode::ArrowDown) {
            camera_object.transform.position -= up * move_speed * delta;
        }

        camera.target = camera_object.transform.position + direction;
    }

    pub unsafe fn destroy(&mut self) {
        self.renderer.destroy();
    }

    fn prepare_renderer(&mut self) {
        let render_items = self
            .world
            .objects()
            .iter()
            .filter_map(|object| {
                let mesh_renderer = object.mesh_renderer.as_ref()?;

                Some(RenderItem {
                    mesh_index: mesh_renderer.mesh.0,
                    transform: object.transform.clone(),
                    is_visible: object.get_visible(),
                })
            })
            .collect();

        self.renderer.set_render_items(render_items);

        // send camera to vulkan
        if let Some(camera_object) = self.world.active_camera() {
            if let Some(camera) = camera_object.camera.as_ref() {
                self.renderer.set_camera(RenderCamera {
                    position: camera_object.transform.position,
                    target: camera.target,
                    up: vec3(0.0, 0.0, 1.0),
                    fov_y: camera.fov_y,
                    near: camera.near,
                    far: camera.far,
                });
            }
        }
    }
}
