use anyhow::Result;
use cgmath::{InnerSpace, vec2, vec3};
use renderer_vulkan::{RenderCamera, RenderItem, VulkanRenderer};
use turbo_math::Transform;
use winit::event::{MouseButton, WindowEvent};
use winit::keyboard::KeyCode;
use winit::window::Window;

use crate::primitive::{
    PrimitiveMesh, PrimitiveShape, PrimitiveType,
    create_primitive_mesh, spawn_primitive_from_mesh,
    update_primitive_mesh,
    spawn_triangle,
};

use crate::world::{
    EntityId,
};

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
    model_mesh: MeshHandle,
    pub primitive_meshes: Vec<PrimitiveMesh>,
    positions: Vec<Vec3>,
}

impl App {
    pub unsafe fn create(window: &Window) -> Result<Self> {
        let mut renderer = VulkanRenderer::create(window)?;
        let mut world = World::default();

        // intialize_mesh
        let model_mesh =
            MeshHandle(renderer.load_mesh_from_model("assets/models/viking_room.obj")?);

        let primitive_meshes = vec![
            create_primitive_mesh(
                &mut renderer,
                PrimitiveShape::Triangle {
                    points: [
                        vec3(0.0, 0.0, 0.5),
                        vec3(0.0, -0.5, -0.5),
                        vec3(0.0, 0.5, -0.5),
                    ],
                    color: vec3(1.0, 1.0, 1.0),
                },
            )?,
            create_primitive_mesh(
                &mut renderer,
                PrimitiveShape::Rectangle {
                    points: [
                        vec3(0.0, -0.5, 0.5),
                        vec3(0.0, -0.5, -0.5),
                        vec3(0.0, 0.5, -0.5),
                        vec3(0.0, 0.5, 0.5),
                    ],
                    color: vec3(1.0, 1.0, 1.0),
                },
            )?,
            create_primitive_mesh(
                &mut renderer,
                PrimitiveShape::Cube {
                    points: [
                        vec3(0.5, -0.5, 0.5),
                        vec3(0.5, 0.5, 0.5),
                        vec3(-0.5, 0.5, 0.5),
                        vec3(-0.5, -0.5, 0.5),
                        vec3(0.5, -0.5, -0.5),
                        vec3(0.5, 0.5, -0.5),
                        vec3(-0.5, 0.5, -0.5),
                        vec3(-0.5, -0.5, -0.5),
                    ],
                    color: vec3(1.0, 1.0, 1.0),
                },
            )?,
            create_primitive_mesh(
                &mut renderer,
                PrimitiveShape::Circle {
                    radius: 1.0,
                    segments: 32,
                    color: vec3(1.0, 1.0, 1.0),
                },
            )?,
            create_primitive_mesh(
                &mut renderer,
                PrimitiveShape::Polygon {
                    points: vec![
                        vec3(0.0, -0.7, 0.7),
                        vec3(0.0, -0.4, 0.5),
                        vec3(0.0, 0.7, 0.5),
                        vec3(0.0, 0.0, -0.6),
                        vec3(0.0, -0.5, -0.4),
                    ],
                    color: vec3(1.0, 1.0, 1.0),
                },
            )?,
            create_primitive_mesh(
                &mut renderer,
                PrimitiveShape::Sphere { 
                    radius: 1.0, 
                    rings: 32, 
                    segments: 32, 
                    color: vec3(1.0,1.0,1.0)
                }
            )?,
        ];


        let positions = vec![
            vec3(0.0, -1.25, 1.0),
            vec3(0.0, 1.25, 1.0),
            vec3(0.0, -1.25, -1.0),
            vec3(0.0, 1.25, -1.0),
        ];


        // camera //////////////////
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

        let mut input = Input::default();
        let window_size = window.inner_size();
        input.set_window_size(vec2(window_size.width as f32, window_size.height as f32));

        let mut app = Self {
            renderer,
            world,
            input,
            time: Time::default(),
            model_mesh,
            primitive_meshes,
            positions,
        };

        unsafe{
            let id=app.spawn_triangle(
            vec3(0.0,-0.2,-0.5),
            vec3(0.0,0.5,0.2),
            vec3(0.0,0.0,0.5),
            vec3(1.0,1.0,1.0),
            )?;

        }
        let triangle_count=app
            .primitive_meshes
            .iter()
            .filter(|mesh| mesh.primitive_type==PrimitiveType::Triangle)
            .count();
        assert!(triangle_count==2);

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
        self.process_input()?;
        self.update_world()?;
        self.update_camera();

        self.prepare_renderer();
        self.input.clear_transitions();
        Ok(())
    }

    fn update_world(&mut self) -> Result<()> {
        self.world.update(self.time.delta_seconds())
    }

    fn process_input(&mut self) -> Result<()> {
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
                .filter(|object| {
                    object
                        .mesh_renderer
                        .as_ref()
                        .is_some_and(|mesh_renderer| mesh_renderer.mesh == self.model_mesh)
                })
                .count();

            if self.positions.len() > index {
                let id = self.world.spawn(
                    Transform {
                        position: self.positions[index],
                        ..Default::default()
                    },
                    Some(MeshRenderer {
                        mesh: self.model_mesh,
                    }),
                    None,
                    vec3(20.0, 0.0, 0.0),
                );
            }
        }

        if self.input.key_pressed(KeyCode::KeyT) {
            let position = self.mouse_position_on_spawn_plane();
            if let Some(mesh) = self.primitive_handle(PrimitiveType::Triangle) {
                let _id = spawn_primitive_from_mesh(
                    &mut self.world,
                    mesh,
                    Transform {
                        position,
                        ..Default::default()
                    },
                );
            }
        }

        if self.input.key_pressed(KeyCode::KeyR) {
            let position = self.mouse_position_on_spawn_plane();
            if let Some(mesh) = self.primitive_handle(PrimitiveType::Rectangle) {
                let _id = spawn_primitive_from_mesh(
                    &mut self.world,
                    mesh,
                    Transform {
                        position,
                        ..Default::default()
                    },
                );
            }
        }

        if self.input.key_pressed(KeyCode::KeyC) {
            let position = self.mouse_position_on_spawn_plane();
            if let Some(mesh) = self.primitive_handle(PrimitiveType::Cube) {
                let _id = spawn_primitive_from_mesh(
                    &mut self.world,
                    mesh,
                    Transform {
                        position,
                        ..Default::default()
                    },
                );
            }
        }

        if self.input.key_pressed(KeyCode::KeyI) {
            let position = self.mouse_position_on_spawn_plane();
            if let Some(mesh) = self.primitive_handle(PrimitiveType::Circle) {
                let _id = spawn_primitive_from_mesh(
                    &mut self.world,
                    mesh,
                    Transform {
                        position,
                        ..Default::default()
                    },
                );
            }
        }

        if self.input.key_pressed(KeyCode::KeyP) {
            let position = self.mouse_position_on_spawn_plane();
            if let Some(mesh) = self.primitive_handle(PrimitiveType::Polygon) {
                let _id = spawn_primitive_from_mesh(
                    &mut self.world,
                    mesh,
                    Transform {
                        position,
                        ..Default::default()
                    },
                );
            }
        }

        if self.input.key_pressed(KeyCode::KeyE){
            let position=self.mouse_position_on_spawn_plane();
            if let Some(mesh)=self.primitive_handle(PrimitiveType::Sphere){
                let _id=spawn_primitive_from_mesh(
                    &mut self.world,
                    mesh,
                    Transform {
                        position,
                        ..Default::default()
                    }
                );
            }
        }

        // change all polygons' figure
        if self.input.key_pressed(KeyCode::KeyU) {
            if let Some(mesh) = self.primitive_mesh(PrimitiveType::Polygon) {
                unsafe {
                    update_primitive_mesh(
                        &mut self.renderer,
                        mesh,
                        PrimitiveShape::Polygon {
                            points: vec![
                                vec3(0.0, -0.7, 0.3),
                                vec3(0.0, -0.4, 0.2),
                                vec3(0.0, 0.7, 0.5),
                                vec3(0.0, 0.2, -0.2),
                                vec3(0.0, -0.5, -0.45),
                            ],
                            color: vec3(1.0, 1.0, 1.0),
                        },
                    )?;
                }
            }
            if let Some(mesh)=self.primitive_mesh(PrimitiveType::Sphere){
                unsafe{
                    update_primitive_mesh(
                        &mut self.renderer,
                        mesh,
                        PrimitiveShape::Sphere { 
                            radius: 2.0, 
                            rings: 20, 
                            segments: 20, 
                            color: vec3(1.0,1.0,1.0), 
                        },
                    )?;
                }
            }
        }

        Ok(())
    }

    fn primitive_handle(&self, primitive_type: PrimitiveType) -> Option<MeshHandle> {
        self.primitive_mesh(primitive_type).map(|mesh| mesh.handle)
    }

    fn primitive_mesh(&self, primitive_type: PrimitiveType) -> Option<PrimitiveMesh> {
        self.primitive_meshes
            .iter()
            .find(|mesh| mesh.primitive_type == primitive_type)
            .copied()
    }

    fn mouse_position_on_spawn_plane(&self) -> Vec3 {
        let mouse = self.input.mouse_position();
        let window_size = self.input.window_size();
        let width = window_size.x.max(1.0);
        let height = window_size.y.max(1.0);
        let aspect = width / height;
        let world_height = 4.0;

        let x = mouse.x / width - 0.5;
        let y = 0.5 - mouse.y / height;

        vec3(0.0, x * world_height * aspect, y * world_height)
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
        let left = vec3(-direction.y, direction.x, 0.0).normalize();
        let up = vec3(0.0, 0.0, 1.0);

        if self.input.key_down(KeyCode::KeyW) {
            camera_object.transform.position += direction * move_speed * delta;
        }
        if self.input.key_down(KeyCode::KeyS) {
            camera_object.transform.position -= direction * move_speed * delta;
        }
        if self.input.key_down(KeyCode::KeyA) {
            camera_object.transform.position += left * move_speed * delta;
        }
        if self.input.key_down(KeyCode::KeyD) {
            camera_object.transform.position -= left * move_speed * delta;
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

    pub unsafe fn spawn_triangle(
        &mut self,
        p0: Vec3,
        p1: Vec3,
        p2: Vec3,
        color: Vec3,
    )->Result<EntityId>{
        spawn_triangle(
            &mut self.world,
            &mut self.renderer,
            &mut self.primitive_meshes,
            p0,
            p1,
            p2,
            color,
        )
    }
}

// test ///////////////////////////////////////////////////
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn created_world_object_count_matches_created_entity_id_count() {
        let mut world = World::default();
        let triangle_mesh = MeshHandle(0);
        let rectangle_mesh = MeshHandle(1);
        let cube_mesh = MeshHandle(2);

        let ids = vec![
            world.spawn(
                Transform::default(),
                None,
                Some(CameraComponent {
                    target: vec3(0.0, 0.0, 0.0),
                    fov_y: 45.0,
                    near: 0.1,
                    far: 100.0,
                    yaw: 0.0,
                    pitch: 0.0,
                }),
                vec3(0.0, 0.0, 0.0),
            ),
            spawn_primitive_from_mesh(&mut world, triangle_mesh, Transform::default()).unwrap(),
            spawn_primitive_from_mesh(&mut world, rectangle_mesh, Transform::default()).unwrap(),
            spawn_primitive_from_mesh(&mut world, cube_mesh, Transform::default()).unwrap(),
        ];

        assert_eq!(world.objects().len(), ids.len());
        assert!(ids.iter().all(|id| world.get(*id).is_some()));
    }
}
