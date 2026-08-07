use anyhow::{Result, anyhow};
use cgmath::{InnerSpace, vec2, vec3};
use renderer_vulkan::{
    MeshHandle, PipelineKey, RenderCamera, RenderItem, TextureHandle,
    VulkanRenderer,
};
#[cfg(test)]
use renderer_vulkan::VertexLayout;
use std::collections::HashMap;
use turbo_math::Transform;
use winit::event::{MouseButton, WindowEvent};
use winit::keyboard::KeyCode;
use winit::window::Window;

pub const DEFAULT_TEXTURE: TextureHandle = TextureHandle(0);

use crate::primitive::{
    PrimitiveMesh, PrimitiveShape, PrimitiveType, 
    create_primitive_debug_line, create_primitive_mesh3d, create_primitive_lit3d, create_primitive_ui2d,
    spawn_circle_with_material, spawn_cube_with_material, spawn_line_with_material, 
    spawn_polygon_with_material, spawn_rectangle_with_material, 
    spawn_sphere_with_material, spawn_triangle_with_material, 
    update_primitive_mesh, spawn_primitive_from_mesh, 
};

use crate::world::EntityId;

use super::CameraComponent;
use super::Input;
use super::Material;
use super::MeshRenderer;
use super::Time;
use super::World;

pub type Vec3 = cgmath::Vector3<f32>;
pub type Vec2 = cgmath::Vector2<f32>;

pub struct App {
    pub renderer: VulkanRenderer,
    pub world: World,
    pub input: Input,
    pub time: Time,
    models: HashMap<String, MeshHandle>,
    pub primitive_meshes: Vec<PrimitiveMesh>,
    textures: HashMap<String, TextureHandle>,
    positions: Vec<Vec3>,
}

impl App {
    pub unsafe fn create(window: &Window) -> Result<Self> {
        let mut renderer = VulkanRenderer::create(window)?;
        let mut world = World::default();

        // load data
        let models = load_models(&mut renderer)?;
        let primitive_meshes = create_primitive_meshes(&mut renderer)?;
        let textures = load_textures(&mut renderer)?;

        let positions = vec![
            vec3(0.0, -1.25, 1.0),
            vec3(0.0, 1.25, 1.0),
            vec3(0.0, -1.25, -1.0),
            vec3(0.0, 1.25, -1.0),
        ];

        // camera //////////////////
        world.spawn(
            Transform {
                position: vec3(-1.0, 0.0, 0.0),
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
            models,
            primitive_meshes,
            textures,
            positions,
        };

        #[cfg(debug_assertions)]
        {
            // create primitive ////////////////////////////
            unsafe {
                let triangle_id0 = app.spawn_triangle_3d(
                    vec3(5.0, -0.2, -0.5),
                    vec3(5.0, 0.5, 0.2),
                    vec3(5.0, 0.0, 0.5),
                    vec3(0.0, 1.0, 1.0),
                    0.5,
                    None,
                    PipelineKey::Mesh3D,
                )?;
                let triangle_id1 = app.spawn_triangle_3d(
                    vec3(0.0, -0.2, -0.5),
                    vec3(0.0, 0.5, 0.2),
                    vec3(0.0, 0.0, 0.5),
                    vec3(0.0, 1.0, 1.0),
                    0.5,
                    None,
                    PipelineKey::Transparent3D,
                )?;
                let triangle_id2=app.spawn_triangle_3d(
                    vec3(-10.0, -0.2, -0.5),
                    vec3(-10.0, 0.5, 0.2),
                    vec3(-10.0, 0.0, 0.5),
                    vec3(0.0, 1.0, 1.0),
                    0.5,
                    None,
                    PipelineKey::DebugLine3D,
                    
                )?;
                let triangle_id3=app.spawn_triangle_3d(
                    vec3(-5.0, -0.2, -0.5),
                    vec3(-5.0, 0.5, 0.2),
                    vec3(-5.0, 0.0, 0.5),
                    vec3(0.0, 1.0, 1.0),
                    1.0,
                    None,
                    PipelineKey::Lit3D,
                )?;
                let triangle_ui2d=app.spawn_triangle_3d(
                    vec3(0.0,-0.6,0.5),
                    vec3(0.0,-0.7,0.7),
                    vec3(0.0,-0.8,0.5),
                    vec3(0.0,1.0,1.0),
                    0.4,
                    None,
                    PipelineKey::Ui2D,
                )?;

                let rectangle_id0 = app.spawn_rectangle_3d(
                    vec3(5.0, 0.5, 0.5),
                    0.3,
                    0.3,
                    vec3(0.0, 0.0, 0.0),
                    vec3(1.0, 0.0, 1.0),
                    0.5,
                    None,
                    PipelineKey::Mesh3D,
                )?;
                let rectangle_id1 = app.spawn_rectangle_3d(
                    vec3(0.0, 0.5, 0.5),
                    0.3,
                    0.3,
                    vec3(0.0, 0.0, 0.0),
                    vec3(1.0, 0.0, 1.0),
                    0.5,
                    None,
                    PipelineKey::Transparent3D,
                )?;
                let rectangle_id2 = app.spawn_rectangle_3d(
                    vec3(-10.0, 0.5, 0.5),
                    0.3,
                    0.3,
                    vec3(0.0, 0.0, 0.0),
                    vec3(1.0, 0.0, 1.0),
                    0.5,
                    None,
                    PipelineKey::DebugLine3D,
                )?;
                let rectangle_id3 = app.spawn_rectangle_3d(
                    vec3(-5.0, 0.5, 0.5),
                    0.3,
                    0.3,
                    vec3(0.0, 0.0, 0.0),
                    vec3(1.0, 0.0, 1.0),
                    1.0,
                    None,
                    PipelineKey::Lit3D,
                )?;

                let cube_id0 = app.spawn_cube_3d(
                    vec3(5.0, 1.0, 1.0),
                    1.0,
                    vec3(0.0, 0.0, 0.0),
                    vec3(1.0, 1.0, 0.0),
                    0.5,
                    None,
                    PipelineKey::Mesh3D,
                )?;
                let cube_id1 = app.spawn_cube_3d(
                    vec3(0.0, 1.0, 1.0),
                    1.0,
                    vec3(0.0, 0.0, 0.0),
                    vec3(1.0, 1.0, 0.0),
                    0.5,
                    None,
                    PipelineKey::Transparent3D,
                )?;
                let cube_id2 = app.spawn_cube_3d(
                    vec3(-10.0, 1.0, 1.0),
                    1.0,
                    vec3(0.0, 0.0, 0.0),
                    vec3(1.0, 1.0, 0.0),
                    0.5,
                    None,
                    PipelineKey::DebugLine3D,
                )?;
                let cube_id3 = app.spawn_cube_3d(
                    vec3(-5.0, 1.0, 1.0),
                    1.0,
                    vec3(0.0, 45.0, 0.0),
                    vec3(1.0, 1.0, 0.0),
                    1.0,
                    None,
                    PipelineKey::Lit3D,
                )?;

                let circle_id0 = app.spawn_circle_3d(
                    vec3(5.0, 2.0, 1.0),
                    1.0,
                    32,
                    vec3(0.0, 0.0, 1.0),
                    0.5,
                    None,
                    PipelineKey::Mesh3D,
                )?;
                let circle_id1 = app.spawn_circle_3d(
                    vec3(0.0, 2.0, 1.0),
                    1.0,
                    32,
                    vec3(0.0, 0.0, 1.0),
                    0.5,
                    None,
                    PipelineKey::Transparent3D,
                )?;
                let circle_id2 = app.spawn_circle_3d(
                    vec3(-10.0, 2.0, 1.0),
                    1.0,
                    32,
                    vec3(0.0, 0.0, 1.0),
                    0.5,
                    None,
                    PipelineKey::DebugLine3D,
                )?;
                let circle_id3 = app.spawn_circle_3d(
                    vec3(-5.0, 2.0, 1.0),
                    1.0,
                    32,
                    vec3(0.0, 0.0, 1.0),
                    1.0,
                    None,
                    PipelineKey::Lit3D,
                )?;
                /* 
                let circle_ui2d = app.spawn_circle(
                    vec3(0.0, 0.5, 0.5),
                    0.2,
                    32,
                    vec3(0.0, 0.0, 1.0),
                    1.0,
                    PipelineKey::Ui2D,
                )?;
                */

                let polygon_id0 = app.spawn_polygon_3d(
                    vec![
                        vec3(5.0, -0.4, -1.0),
                        vec3(5.0, -0.2, 0.0),
                        vec3(5.0, 0.5, -0.3),
                        vec3(5.0, 0.3, 0.2),
                        vec3(5.0, 0.0, 1.0),
                        vec3(5.0, -0.1, 1.2),
                    ],
                    vec3(0.0, 1.0, 0.0),
                    0.5,
                    None,
                    PipelineKey::Mesh3D,
                )?;
                let polygon_id1 = app.spawn_polygon_3d(
                    vec![
                        vec3(0.0, -0.4, -1.0),
                        vec3(0.0, -0.2, 0.0),
                        vec3(0.0, 0.5, -0.3),
                        vec3(0.0, 0.3, 0.2),
                        vec3(0.0, 0.0, 1.0),
                        vec3(0.0, -0.1, 1.2),
                    ],
                    vec3(0.0, 1.0, 0.0),
                    0.5,
                    None,
                    PipelineKey::Transparent3D,
                )?;
                let polygon_id2 = app.spawn_polygon_3d(
                    vec![
                        vec3(-10.0, -0.4, -1.0),
                        vec3(-10.0, -0.2, 0.0),
                        vec3(-10.0, 0.5, -0.3),
                        vec3(-10.0, 0.3, 0.2),
                        vec3(-10.0, 0.0, 1.0),
                        vec3(-10.0, -0.1, 1.2),
                    ],
                    vec3(0.0, 1.0, 0.0),
                    0.5,
                    None,
                    PipelineKey::DebugLine3D,
                )?;
                let polygon_id3 = app.spawn_polygon_3d(
                    vec![
                        vec3(-5.0, -0.4, -1.0),
                        vec3(-5.0, -0.2, 0.0),
                        vec3(-5.0, 0.5, -0.3),
                        vec3(-5.0, 0.3, 0.2),
                        vec3(-5.0, 0.0, 1.0),
                        vec3(-5.0, -0.1, 1.2),
                    ],
                    vec3(0.0, 1.0, 0.0),
                    1.0,
                    None,
                    PipelineKey::Lit3D,
                )?;

                let sphere_id0 = app.spawn_sphere_3d(
                    vec3(5.0, -1.0, 0.0),
                    0.5,
                    16,
                    16,
                    vec3(1.0, 0.0, 0.0),
                    0.5,
                    None,
                    PipelineKey::Mesh3D,
                )?;
                let sphere_id1 = app.spawn_sphere_3d(
                    vec3(0.0, -1.0, 0.0),
                    0.5,
                    16,
                    16,
                    vec3(1.0, 0.0, 0.0),
                    0.5,
                    None,
                    PipelineKey::Transparent3D,
                )?;
                let sphere_id2 = app.spawn_sphere_3d(
                    vec3(-10.0, -1.0, 0.0),
                    0.5,
                    16,
                    16,
                    vec3(1.0, 0.0, 0.0),
                    0.5,
                    None,
                    PipelineKey::DebugLine3D,
                )?;
                let sphere_id3 = app.spawn_sphere_3d(
                    vec3(-5.0, -1.0, 0.0),
                    0.5,
                    16,
                    16,
                    vec3(1.0, 0.0, 0.0),
                    1.0,
                    None,
                    PipelineKey::Lit3D,
                )?;

                let line_id1 = app.spawn_line_3d(
                    vec3(0.0, -20.0, 0.0),
                    vec3(0.0, 20.0, 0.0),
                    vec3(1.0, 0.0, 1.0),
                    1.0,
                )?;
                let line_id2 = app.spawn_line_3d(
                    vec3(0.0, 0.0, -20.0),
                    vec3(0.0, 0.0, 20.0),
                    vec3(1.0, 1.0, 0.0),
                    1.0,
                )?;
                let line_id3 = app.spawn_line_3d(
                    vec3(-20.0, 0.0, 0.0),
                    vec3(20.0, 0.0, 0.0),
                    vec3(0.0, 1.0, 1.0),
                    1.0,
                )?;
            }
        }
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
            let viking_room_mesh3d = self.use_model("viking_room")?;
            let viking_room_debug_line = self.use_model("viking_room_debug_line")?;
            let viking_room_lit3d = self.use_model("viking_room_lit3d")?;
            let viking_texture = self.use_texture("viking_room");
            let viking_meshes = [
                viking_room_mesh3d,
                viking_room_debug_line,
                viking_room_lit3d,
            ];
            let index = self
                .world
                .objects()
                .iter()
                .filter(|object| {
                    object
                        .mesh_renderer
                        .as_ref()
                        .is_some_and(|mesh_renderer| viking_meshes.contains(&mesh_renderer.mesh))
                })
                .count();

            if self.positions.len() > index {
                let variants = [
                    (viking_room_mesh3d, PipelineKey::Mesh3D, 1.0),
                    (viking_room_debug_line, PipelineKey::DebugLine3D, 1.0),
                    (viking_room_mesh3d, PipelineKey::Transparent3D, 0.5),
                    (viking_room_lit3d, PipelineKey::Lit3D, 1.0),
                ];
                let (mesh, pipeline_key, alpha) = variants[index];
                match MeshRenderer::new(
                    mesh,
                    Material {
                        color: vec3(1.0, 1.0, 1.0),
                        alpha,
                        use_texture: true,
                        texture: viking_texture,
                        pipeline_key,
                    },
                ){
                    Ok(mesh_renderer) =>{
                        let id = self.world.spawn(
                            Transform {
                                position: self.positions[index],
                                ..Default::default()
                            },
                            Some(mesh_renderer),
                            None,
                            vec3(20.0, 0.0, 0.0),
                        );
                    }
                    Err(e) =>{
                        log::error!("Failed to spawn triangle primitive: {e:?}");
                    }
                };

            }
        }

        if self.input.key_pressed(KeyCode::KeyT) {
            let position = self.mouse_position_on_spawn_plane();
            let face_texture = self.use_texture("face");
            if let Some(mesh) = self.primitive_handle(PrimitiveType::Triangle) {
                if let Err(e) = spawn_primitive_from_mesh(
                    &mut self.world,
                    mesh,
                    Material {
                        color: vec3(1.0, 1.0, 1.0),
                        use_texture: true,
                        texture: face_texture,
                        pipeline_key: PipelineKey::Lit3D,
                        ..Default::default()
                    },
                    Transform {
                        position,
                        ..Default::default()
                    },
                ) {
                    log::error!("Failed to spawn triangle primitive: {e:?}");
                }
            }
        }

        if self.input.key_pressed(KeyCode::KeyR) {
            let position = self.mouse_position_on_spawn_plane();
            let face_texture=self.use_texture("face");
            if let Some(mesh) = self.primitive_handle(PrimitiveType::Rectangle) {
                if let Err(e) = spawn_primitive_from_mesh(
                    &mut self.world,
                    mesh,
                    Material {
                        color: vec3(1.0, 1.0, 1.0),
                        use_texture: true,
                        texture: face_texture,
                        pipeline_key: PipelineKey::Ui2D,
                        ..Default::default()
                    },
                    Transform {
                        position,
                        ..Default::default()
                    },
                ) {
                    log::error!("Failed to spawn rectangle primitive: {e:?}");
                }
            }
        }

        if self.input.key_pressed(KeyCode::KeyC) {
            let position = self.mouse_position_on_spawn_plane();
            if let Some(mesh) = self.primitive_handle(PrimitiveType::Cube) {
                if let Err(e) = spawn_primitive_from_mesh(
                    &mut self.world,
                    mesh,
                    Material {
                        color: vec3(1.0, 1.0, 1.0),
                        use_texture: true,
                        pipeline_key: PipelineKey::DebugLine3D,
                        ..Default::default()
                    },
                    Transform {
                        position,
                        ..Default::default()
                    },
                ) {
                    log::error!("Failed to spawn cube primitive: {e:?}");
                }
            }
        }

        if self.input.key_pressed(KeyCode::KeyI) {
            let position = self.mouse_position_on_spawn_plane();
            if let Some(mesh) = self.primitive_handle(PrimitiveType::Circle) {
                if let Err(e) = spawn_primitive_from_mesh(
                    &mut self.world,
                    mesh,
                    Material {
                        color: vec3(1.0, 1.0, 1.0),
                        use_texture: true,
                        pipeline_key: PipelineKey::Mesh3D,
                        ..Default::default()
                    },
                    Transform {
                        position,
                        ..Default::default()
                    },
                ) {
                    log::error!("Failed to spawn circle primitive: {e:?}");
                }
            }
        }

        if self.input.key_pressed(KeyCode::KeyP) {
            let position = self.mouse_position_on_spawn_plane();
            if let Some(mesh) = self.primitive_handle(PrimitiveType::Polygon) {
                if let Err(e) = spawn_primitive_from_mesh(
                    &mut self.world,
                    mesh,
                    Material {
                        color: vec3(1.0, 1.0, 1.0),
                        use_texture: true,
                        pipeline_key: PipelineKey::Mesh3D,
                        ..Default::default()
                    },
                    Transform {
                        position,
                        ..Default::default()
                    },
                ) {
                    log::error!("Failed to spawn polygon primitive: {e:?}");
                }
            }
        }

        if self.input.key_pressed(KeyCode::KeyE) {
            let position = self.mouse_position_on_spawn_plane();
            if let Some(mesh) = self.primitive_handle(PrimitiveType::Sphere) {
                if let Err(e) = spawn_primitive_from_mesh(
                    &mut self.world,
                    mesh,
                    Material {
                        color: vec3(1.0, 1.0, 1.0),
                        use_texture: true,
                        pipeline_key: PipelineKey::Lit3D,
                        ..Default::default()
                    },
                    Transform {
                        position,
                        ..Default::default()
                    },
                ) {
                    log::error!("Failed to spawn sphere primitive: {e:?}");
                }
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
                            color: vec3(1.0, 0.0, 0.0),
                        },
                    )?;
                }
            }
            if let Some(mesh) = self.primitive_mesh(PrimitiveType::Sphere) {
                unsafe {
                    update_primitive_mesh(
                        &mut self.renderer,
                        mesh,
                        PrimitiveShape::Sphere {
                            radius: 2.0,
                            rings: 20,
                            segments: 20,
                            color: vec3(0.0, 1.0, 1.0),
                        },
                    )?;
                }
            }
            if let Some(mesh) = self.primitive_mesh(PrimitiveType::Rectangle){
                unsafe{
                    update_primitive_mesh(
                        &mut self.renderer,
                        mesh,
                        PrimitiveShape::Rectangle { 
                            points:[
                                vec3(0.0, -0.2,0.2),
                                vec3(0.0, -0.2,-0.2),
                                vec3(0.0, 0.2,-0.2),
                                vec3(0.0,0.2,0.2)
                            ], 
        
                            color: vec3(1.0,1.0,1.0)
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

    fn use_texture(&self, name: &str) -> TextureHandle {
        self.textures.get(name).copied().unwrap_or(DEFAULT_TEXTURE)
    }

    fn use_model(&self, name: &str) -> Result<MeshHandle> {
        self.models
            .get(name)
            .copied()
            .ok_or_else(|| anyhow!("Model not found: {name}"))
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
                    mesh_index: mesh_renderer.mesh,
                    transform: object.transform.clone(),
                    alpha: mesh_renderer.material.alpha,
                    material_color: mesh_renderer.material.color,
                    use_texture: mesh_renderer.material.use_texture,
                    texture_index: mesh_renderer.material.texture,
                    pipeline_key: mesh_renderer.material.pipeline_key,
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

    // spawn primitive shape ////////////////////////////
    fn material(
        color: Vec3,
        alpha: f32,
        texture: Option<TextureHandle>,
        pipeline_key: PipelineKey,
    ) -> Material {
        let use_texture = texture.is_some();
        let texture = texture.unwrap_or(DEFAULT_TEXTURE);

        Material {
            color,
            alpha,
            use_texture,
            texture,
            pipeline_key,
        }
    }

    pub unsafe fn spawn_triangle_3d(
        &mut self,
        p0: Vec3,
        p1: Vec3,
        p2: Vec3,
        color: Vec3,
        alpha: f32,
        texture: Option<TextureHandle>,
        pipeline_key: PipelineKey,
    ) -> Result<EntityId> {
        let material = Self::material(color, alpha, texture, pipeline_key);
        spawn_triangle_with_material(
            &mut self.world,
            &mut self.renderer,
            &mut self.primitive_meshes,
            p0,
            p1,
            p2,
            material,
        )
    }

    pub unsafe fn spawn_triangle_2d(
        &mut self,
        p0: Vec2,
        p1: Vec2,
        p2: Vec2,
        color: Vec3,
        alpha: f32,
        texture: Option<TextureHandle>,
    ) -> Result<EntityId>{
        let material = Self::material(color, alpha, texture, PipelineKey::Ui2D);
        spawn_triangle_with_material(
            &mut self.world,
            &mut self.renderer,
            &mut self.primitive_meshes,
            vec3(0.0,p0.x,p0.y),
            vec3(0.0,p1.x,p1.y),
            vec3(0.0,p2.x,p2.y),
            material
        )
    }

    pub unsafe fn spawn_rectangle_3d(
        &mut self,
        pos: Vec3,
        width: f32,
        height: f32,
        rotation: Vec3,
        color: Vec3,
        alpha: f32,
        texture: Option<TextureHandle>,
        pipeline_key: PipelineKey,
    ) -> Result<EntityId> {
        let material = Self::material(color, alpha, texture, pipeline_key);

        spawn_rectangle_with_material(
           &mut self.world,
           &mut self.renderer,
           &mut self.primitive_meshes,
           pos,
           width,
           height,
           rotation,
           material,
        )
    }

    pub unsafe fn spawn_rectangle_2d(
        &mut self,
        pos: Vec2,
        width: f32,
        height: f32,
        rotation: f32,
        color: Vec3,
        alpha: f32,
        texture: Option<TextureHandle>,
    ) -> Result<EntityId>{
        let material = Self::material(color, alpha, texture, PipelineKey::Ui2D);

        spawn_rectangle_with_material(
            &mut self.world,
            &mut self.renderer,
            &mut self.primitive_meshes,
            vec3(0.0,pos.x,pos.y),
            width,
            height,
            vec3(rotation, 0.0, 0.0),
            material
        )
    }

    pub unsafe fn spawn_cube_3d(
        &mut self,
        pos: Vec3,
        length: f32,
        rotation: Vec3,
        color: Vec3,
        alpha: f32,
        texture: Option<TextureHandle>,
        pipeline_key: PipelineKey,
    ) -> Result<EntityId> {
        let material = Self::material(color, alpha, texture, pipeline_key);
        spawn_cube_with_material(
            &mut self.world,
            &mut self.renderer,
            &mut self.primitive_meshes,
            pos,
            length,
            rotation,
            material,
        )
    }

    pub unsafe fn spawn_circle_3d(
        &mut self,
        pos: Vec3,
        radius: f32,
        segments: u32,
        color: Vec3,
        alpha: f32,
        texture: Option<TextureHandle>,
        pipeline_key: PipelineKey,
    ) -> Result<EntityId> {
        let material = Self::material(color, alpha, texture, pipeline_key);
        spawn_circle_with_material(
            &mut self.world,
            &mut self.renderer,
            &mut self.primitive_meshes,
            pos,
            radius,
            segments,
            material,
        )
    }

    pub unsafe fn spawn_circle_2d(
        &mut self,
        pos: Vec2,
        radius: f32,
        segments: u32,
        color: Vec3,
        alpha: f32,
        texture: Option<TextureHandle>,
    ) -> Result<EntityId> {
        let material = Self::material(color, alpha, texture, PipelineKey::Ui2D);
        spawn_circle_with_material(
            &mut self.world,
            &mut self.renderer,
            &mut self.primitive_meshes,
            vec3(0.0,pos.x,pos.y),
            radius,
            segments,
            material,
        )
    }

    pub unsafe fn spawn_polygon_3d(
        &mut self,
        points: Vec<Vec3>,
        color: Vec3,
        alpha: f32,
        texture: Option<TextureHandle>,
        pipeline_key: PipelineKey,
    ) -> Result<EntityId> {
        let material = Self::material(color, alpha, texture, pipeline_key);
        spawn_polygon_with_material(
            &mut self.world,
            &mut self.renderer,
            &mut self.primitive_meshes,
            points,
            material,
        )
    }
    pub unsafe fn spawn_polygon_2d(
        &mut self,
        points: Vec<Vec2>,
        color: Vec3,
        alpha: f32,
        texture: Option<TextureHandle>,
    ) -> Result<EntityId> {
        let material = Self::material(color, alpha, texture, PipelineKey::Ui2D);
        let points = points
            .into_iter()
            .map(|p| vec3(0.0, p.x, p.y))
            .collect();

        spawn_polygon_with_material(
            &mut self.world,
            &mut self.renderer,
            &mut self.primitive_meshes,
            points,
            material,
        )
    }

    pub unsafe fn spawn_sphere_3d(
        &mut self,
        center: Vec3,
        radius: f32,
        rings: u32,
        segments: u32,
        color: Vec3,
        alpha: f32,
        texture: Option<TextureHandle>,
        pipeline_key: PipelineKey
    ) -> Result<EntityId> {
        let material = Self::material(color, alpha, texture, pipeline_key);
        spawn_sphere_with_material(
            &mut self.world,
            &mut self.renderer,
            &mut self.primitive_meshes,
            center,
            radius,
            rings,
            segments,
            material,
        )
    }

    pub unsafe fn spawn_line_3d(
        &mut self,
        pos0: Vec3,
        pos1: Vec3, 
        color: Vec3,
        alpha: f32,
    ) -> Result<EntityId> {
        let material = Self::material(color, alpha, None, PipelineKey::DebugLine3D);
        spawn_line_with_material(
            &mut self.world,
            &mut self.renderer,
            &mut self.primitive_meshes,
            pos0,
            pos1,
            material,
        )
    }


    pub unsafe fn spawn_line_2d(
        &mut self,
        pos0: Vec2,
        pos1: Vec2,
        color: Vec3,
        width: f32,
        alpha: f32,
    ) -> Result<EntityId>{
        let material = Self::material(color, alpha, None, PipelineKey::Ui2D);
        let from=vec3(0.0,pos0.x,pos0.y);
        let to=vec3(0.0,pos1.x,pos1.y);
        let center=(from+to)/2.0;
        let delta = to - from;
        let length = (delta.y * delta.y + delta.z * delta.z).sqrt();
        let rotation=vec3((-delta.y).atan2(delta.z).to_degrees(), 0.0, 0.0);
        spawn_rectangle_with_material(
            &mut self.world,
            &mut self.renderer,
            &mut self.primitive_meshes,
            center,
            width,
            length,
            rotation,
            material,
        )
    }

    
}

unsafe fn load_models(renderer: &mut VulkanRenderer) -> Result<HashMap<String, MeshHandle>> {
    let mut models = HashMap::new();

    models.insert(
        "viking_room".to_string(),
        renderer.load_mesh3d_from_model("assets/models/viking_room.obj")?,
    );
    models.insert(
        "viking_room_debug_line".to_string(),
        renderer.load_debug_line_from_model("assets/models/viking_room.obj")?,
    );
    models.insert(
        "viking_room_lit3d".to_string(),
        renderer.load_lit3d_from_model("assets/models/viking_room.obj")?,
    );

    Ok(models)
}

unsafe fn load_textures(renderer: &mut VulkanRenderer) -> Result<HashMap<String, TextureHandle>> {
    let mut textures = HashMap::new();
    textures.insert(
        "viking_room".to_string(),
        renderer.load_texture("assets/textures/viking_room.png")?,
    );
    textures.insert(
        "face".to_string(),
        renderer.load_texture("assets/textures/texture.png")?,
    );

    Ok(textures)
}

unsafe fn create_primitive_meshes(renderer: &mut VulkanRenderer) -> Result<Vec<PrimitiveMesh>> {
    let primitive_meshes = vec![
        create_primitive_lit3d(
            renderer,
            PrimitiveShape::Triangle {
                points: [
                    vec3(0.0, 0.0, 0.5),
                    vec3(0.0, -0.5, -0.5),
                    vec3(0.0, 0.5, -0.5),
                ],
                color: vec3(1.0, 1.0, 1.0),
            },
        )?,
        create_primitive_ui2d(
            renderer,
            PrimitiveShape::Rectangle {
                points: [
                    vec3(0.0, -0.5, 0.5),
                    vec3(0.0, -0.5, -0.5),
                    vec3(0.0, 0.5, -0.5),
                    vec3(0.0, 0.5, 0.5),
                ],
                color: vec3(1.0, 1.0, 0.0),
            },
        )?,
        create_primitive_debug_line(
            renderer,
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
                color: vec3(1.0, 0.0, 0.0),
            },
        )?,
        create_primitive_mesh3d(
            renderer,
            PrimitiveShape::Circle {
                radius: 1.0,
                segments: 32,
                color: vec3(0.0, 1.0, 1.0),
            },
        )?,
        create_primitive_mesh3d(
            renderer,
            PrimitiveShape::Polygon {
                points: vec![
                    vec3(0.0, -0.7, 0.7),
                    vec3(0.0, -0.4, 0.5),
                    vec3(0.0, 0.7, 0.5),
                    vec3(0.0, 0.0, -0.6),
                    vec3(0.0, -0.5, -0.4),
                ],
                color: vec3(0.0, 1.0, 0.0),
            },
        )?,
        create_primitive_lit3d(
            renderer,
            PrimitiveShape::Sphere {
                radius: 1.0,
                rings: 32,
                segments: 32,
                color: vec3(0.0, 0.0, 1.0),
            },
        )?,
    ];

    Ok(primitive_meshes)
}

// test ///////////////////////////////////////////////////
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn created_world_object_count_matches_created_entity_id_count() {
        let mut world = World::default();
        let triangle_mesh = PrimitiveMesh {
            handle: MeshHandle::new(0, VertexLayout::Mesh3D),
            primitive_type: PrimitiveType::Triangle,
            vertex_layout: VertexLayout::Mesh3D,
        };
        let rectangle_mesh = PrimitiveMesh {
            handle: MeshHandle::new(1, VertexLayout::Mesh3D),
            primitive_type: PrimitiveType::Rectangle,
            vertex_layout: VertexLayout::Mesh3D,
        };
        let cube_mesh = PrimitiveMesh {
            handle: MeshHandle::new(2, VertexLayout::Mesh3D),
            primitive_type: PrimitiveType::Cube,
            vertex_layout: VertexLayout::Mesh3D,
        };

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
            spawn_primitive_from_mesh(
                &mut world,
                triangle_mesh.handle,
                Material::default(),
                Transform::default(),
            )
            .unwrap(),
            spawn_primitive_from_mesh(
                &mut world,
                rectangle_mesh.handle,
                Material::default(),
                Transform::default(),
            )
            .unwrap(),
            spawn_primitive_from_mesh(
                &mut world,
                cube_mesh.handle,
                Material::default(),
                Transform::default(),
            )
            .unwrap(),
        ];

        assert_eq!(world.objects().len(), ids.len());
        assert!(ids.iter().all(|id| world.get(*id).is_some()));
    }
}
