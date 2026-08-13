use anyhow::{Result, anyhow};
use cgmath::{vec2, vec3};
use renderer_vulkan::{
    MeshHandle, PipelineKey, SkyboxTextureHandle, TextureHandle, VertexLayout, VulkanRenderer,
};
use std::collections::HashMap;
use turbo_math::Transform;
use winit::event::WindowEvent;
use winit::keyboard::KeyCode;
use winit::window::Window;

pub const DEFAULT_TEXTURE: TextureHandle = TextureHandle(0);
pub const DEFAULT_SKYBOX_TEXTURE: SkyboxTextureHandle = SkyboxTextureHandle(0);

use crate::primitive::{
    PrimitiveMesh, PrimitiveShape, PrimitiveType, build_primitive_source,
    create_primitive_debug_line, create_primitive_lit3d, create_primitive_mesh3d,
    create_primitive_ui2d, spawn_circle_with_material, spawn_cube_with_material,
    spawn_line_with_material, spawn_polygon_with_material, spawn_primitive_from_mesh,
    spawn_rectangle_with_material, spawn_sphere_with_material, spawn_triangle_with_material,
    update_primitive_mesh,
};

use super::{
    Camera, EntityId, Input, InputCommand, InputSystem, InputTrigger, Material, MeshRenderer,
    Scheduler, Time, World,
};

pub type Vec3 = cgmath::Vector3<f32>;
pub type Vec2 = cgmath::Vector2<f32>;

pub struct App {
    pub renderer: VulkanRenderer,
    pub world: World,
    pub input: Input,
    pub time: Time,
    // model
    models: HashMap<String, MeshHandle>,
    pub primitive_meshes: Vec<PrimitiveMesh>,
    textures: HashMap<String, TextureHandle>,
    // skybox
    pub skybox_mesh: Option<MeshHandle>,
    skybox_textures: HashMap<String, SkyboxTextureHandle>,

    positions: Vec<Vec3>,

    // system
    scheduler: Scheduler,
}

impl App {
    pub unsafe fn create(window: &Window) -> Result<Self> {
        let mut renderer = VulkanRenderer::create(window)?;
        let mut world = World::default();

        // load data
        let models = load_models(&mut renderer)?;
        let primitive_meshes = create_primitive_meshes(&mut renderer)?;
        let textures = load_textures(&mut renderer)?;
        let skybox_mesh = Some(create_skybox_mesh(&mut renderer, 20.0)?);
        let skybox_textures = load_skybox_textures(&mut renderer)?;

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
            Some(Camera {
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
        let input_system = Self::create_input_system();
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
            skybox_mesh,
            skybox_textures,
            positions,
            scheduler: Scheduler {
                input_system,
                ..Default::default()
            },
        };

        #[cfg(debug_assertions)]
        {
            app.set_skybox(app.use_skybox_texture("ghost"))?;
            // create primitive ////////////////////////////
            unsafe {
                let face_texture = app.textures.get("face").copied().unwrap_or(DEFAULT_TEXTURE);
                let ghost_texture = app
                    .textures
                    .get("ghost")
                    .copied()
                    .unwrap_or(DEFAULT_TEXTURE);
                let escapee_texture = app
                    .textures
                    .get("escapee")
                    .copied()
                    .unwrap_or(DEFAULT_TEXTURE);
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
                let triangle_id2 = app.spawn_triangle_3d(
                    vec3(-10.0, -0.2, -0.5),
                    vec3(-10.0, 0.5, 0.2),
                    vec3(-10.0, 0.0, 0.5),
                    vec3(0.0, 1.0, 1.0),
                    0.5,
                    None,
                    PipelineKey::DebugLine3D,
                )?;
                let triangle_id3 = app.spawn_triangle_3d(
                    vec3(-5.0, -0.2, -0.5),
                    vec3(-5.0, 0.5, 0.2),
                    vec3(-5.0, 0.0, 0.5),
                    vec3(0.0, 1.0, 1.0),
                    1.0,
                    Some(face_texture),
                    PipelineKey::Lit3D,
                )?;
                let triangle_ui2d = app.spawn_triangle_2d(
                    vec2(-0.6, 0.5),
                    vec2(-0.7, 0.7),
                    vec2(-0.8, 0.5),
                    vec3(0.0, 1.0, 1.0),
                    0.4,
                    Some(face_texture),
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
                    0.8,
                    0.8,
                    vec3(0.0, 0.0, 0.0),
                    vec3(1.0, 0.0, 1.0),
                    1.0,
                    Some(escapee_texture),
                    PipelineKey::Lit3D,
                )?;
                /*let rectangle_ui2d=app.spawn_rectangle_2d(
                    vec2(0.0,0.3),
                    0.5,
                    0.3,
                    45.0,
                    vec3(1.0,0.0,1.0),
                    1.0,
                    Some(face_texture)
                )?;*/

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
                    Some(face_texture),
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
                    Some(face_texture),
                    PipelineKey::Lit3D,
                )?;
                /*let circle_ui2d=app.spawn_circle_2d(
                    vec2(0.5,0.3),
                    0.3,
                    32,
                    vec3(0.0,0.0,1.0),
                    1.0,
                    Some(face_texture),
                )?;*/

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
                    Some(face_texture),
                    PipelineKey::Lit3D,
                )?;
                /*let polygon_ui2d=app.spawn_polygon_2d(
                    vec![
                        vec2(-0.4, -1.0),
                        vec2(-0.2, 0.0),
                        vec2(0.5, -0.3),
                        vec2(0.3, 0.2),
                        vec2(0.0, 1.0),
                        vec2(-0.1, 1.2),
                    ],
                    vec3(0.0,1.0,0.0),
                    1.0,
                    Some(face_texture),
                )?;*/

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
        app.scheduler
            .render_system
            .update(&mut app.world.registry, &mut app.renderer)?;

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

        self.process_input()?;
        self.update_system()?;

        self.input.clear_transitions();
        Ok(())
    }

    fn update_system(&mut self) -> Result<()> {
        let delta_time = self.time.delta_seconds();
        self.scheduler.update(
            &mut self.world.registry,
            &mut self.renderer,
            &self.input,
            delta_time,
        )?;

        Ok(())
    }

    fn process_input(&mut self) -> Result<()> {
        let commands = self.scheduler.input_commands(&self.input);

        for command in commands {
            self.execute_input_command(command)?;
        }

        Ok(())
    }

    fn execute_input_command(&mut self, command: InputCommand) -> Result<()> {
        match command {
            InputCommand::DespawnLast => {
                let id = self.world.registry.entities().last().copied();

                if let Some(id) = id {
                    self.world.despawn(id);
                }
            }
            InputCommand::SpawnVikingRoom => {
                self.spawn_viking_room_from_input()?;
            }
            InputCommand::SpawnPrimitive {
                primitive_type,
                pipeline_key,
                texture_name,
            } => {
                self.spawn_primitive_from_input(primitive_type, pipeline_key, texture_name);
            }
            InputCommand::UpdatePrimitiveMeshes => {
                self.update_primitive_meshes_from_input()?;
            }
        }

        Ok(())
    }

    fn spawn_viking_room_from_input(&mut self) -> Result<()> {
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
            .registry
            .entities()
            .iter()
            .filter(|entity| {
                self.world
                    .registry
                    .get_component::<MeshRenderer>(**entity)
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
            ) {
                Ok(mesh_renderer) => {
                    self.world.spawn(
                        Transform {
                            position: self.positions[index],
                            ..Default::default()
                        },
                        Some(mesh_renderer),
                        None,
                        vec3(20.0, 0.0, 0.0),
                    );
                }
                Err(e) => {
                    log::error!("Failed to spawn triangle primitive: {e:?}");
                }
            };
        }

        Ok(())
    }

    fn spawn_primitive_from_input(
        &mut self,
        primitive_type: PrimitiveType,
        pipeline_key: PipelineKey,
        texture_name: Option<&'static str>,
    ) {
        let position = self.mouse_position_on_spawn_plane();
        let texture = texture_name
            .map(|name| self.use_texture(name))
            .unwrap_or(DEFAULT_TEXTURE);

        if let Some(mesh) = self.primitive_handle(primitive_type) {
            if let Err(e) = spawn_primitive_from_mesh(
                &mut self.world,
                mesh,
                Material {
                    color: vec3(1.0, 1.0, 1.0),
                    use_texture: true,
                    texture,
                    pipeline_key,
                    ..Default::default()
                },
                Transform {
                    position,
                    ..Default::default()
                },
            ) {
                log::error!("Failed to spawn {primitive_type:?} primitive: {e:?}");
            }
        }
    }

    fn update_primitive_meshes_from_input(&mut self) -> Result<()> {
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
        if let Some(mesh) = self.primitive_mesh(PrimitiveType::Rectangle) {
            unsafe {
                update_primitive_mesh(
                    &mut self.renderer,
                    mesh,
                    PrimitiveShape::Rectangle {
                        points: [
                            vec3(0.0, -0.2, 0.2),
                            vec3(0.0, -0.2, -0.2),
                            vec3(0.0, 0.2, -0.2),
                            vec3(0.0, 0.2, 0.2),
                        ],

                        color: vec3(1.0, 1.0, 1.0),
                    },
                )?;
            }
        }

        Ok(())
    }

    fn create_input_system() -> InputSystem {
        InputSystem::new()
            .bind(
                KeyCode::ArrowLeft,
                InputTrigger::Pressed,
                InputCommand::DespawnLast,
            )
            .bind(
                KeyCode::ArrowRight,
                InputTrigger::Pressed,
                InputCommand::SpawnVikingRoom,
            )
            .bind(
                KeyCode::KeyT,
                InputTrigger::Pressed,
                InputCommand::SpawnPrimitive {
                    primitive_type: PrimitiveType::Triangle,
                    pipeline_key: PipelineKey::Lit3D,
                    texture_name: Some("face"),
                },
            )
            .bind(
                KeyCode::KeyR,
                InputTrigger::Pressed,
                InputCommand::SpawnPrimitive {
                    primitive_type: PrimitiveType::Rectangle,
                    pipeline_key: PipelineKey::Ui2D,
                    texture_name: Some("face"),
                },
            )
            .bind(
                KeyCode::KeyC,
                InputTrigger::Pressed,
                InputCommand::SpawnPrimitive {
                    primitive_type: PrimitiveType::Cube,
                    pipeline_key: PipelineKey::DebugLine3D,
                    texture_name: None,
                },
            )
            .bind(
                KeyCode::KeyI,
                InputTrigger::Pressed,
                InputCommand::SpawnPrimitive {
                    primitive_type: PrimitiveType::Circle,
                    pipeline_key: PipelineKey::Mesh3D,
                    texture_name: None,
                },
            )
            .bind(
                KeyCode::KeyP,
                InputTrigger::Pressed,
                InputCommand::SpawnPrimitive {
                    primitive_type: PrimitiveType::Polygon,
                    pipeline_key: PipelineKey::Mesh3D,
                    texture_name: None,
                },
            )
            .bind(
                KeyCode::KeyE,
                InputTrigger::Pressed,
                InputCommand::SpawnPrimitive {
                    primitive_type: PrimitiveType::Sphere,
                    pipeline_key: PipelineKey::Lit3D,
                    texture_name: None,
                },
            )
            .bind(
                KeyCode::KeyU,
                InputTrigger::Pressed,
                InputCommand::UpdatePrimitiveMeshes,
            )
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

    fn use_skybox_texture(&self, name: &str) -> SkyboxTextureHandle {
        self.skybox_textures
            .get(name)
            .copied()
            .unwrap_or(DEFAULT_SKYBOX_TEXTURE)
    }

    pub unsafe fn destroy(&mut self) {
        self.renderer.destroy();
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
    ) -> Result<EntityId> {
        let material = Self::material(color, alpha, texture, PipelineKey::Ui2D);
        spawn_triangle_with_material(
            &mut self.world,
            &mut self.renderer,
            &mut self.primitive_meshes,
            vec3(0.0, p0.x, p0.y),
            vec3(0.0, p1.x, p1.y),
            vec3(0.0, p2.x, p2.y),
            material,
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
    ) -> Result<EntityId> {
        let material = Self::material(color, alpha, texture, PipelineKey::Ui2D);

        spawn_rectangle_with_material(
            &mut self.world,
            &mut self.renderer,
            &mut self.primitive_meshes,
            vec3(0.0, pos.x, pos.y),
            width,
            height,
            vec3(rotation, 0.0, 0.0),
            material,
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
            vec3(0.0, pos.x, pos.y),
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
        let points = points.into_iter().map(|p| vec3(0.0, p.x, p.y)).collect();

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
        pipeline_key: PipelineKey,
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
    ) -> Result<EntityId> {
        let material = Self::material(color, alpha, None, PipelineKey::Ui2D);
        let from = vec3(0.0, pos0.x, pos0.y);
        let to = vec3(0.0, pos1.x, pos1.y);
        let center = (from + to) / 2.0;
        let delta = to - from;
        let length = (delta.y * delta.y + delta.z * delta.z).sqrt();
        let rotation = vec3((-delta.y).atan2(delta.z).to_degrees(), 0.0, 0.0);
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

    pub unsafe fn set_skybox(&mut self, texture: SkyboxTextureHandle) -> Result<()> {
        let mesh = self
            .skybox_mesh
            .ok_or_else(|| anyhow!("Skybox mesh has not been created."))?;

        self.renderer.set_skybox(mesh, texture)
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
    textures.insert(
        "ghost".to_string(),
        renderer.load_texture("assets/textures/ghost.png")?,
    );
    textures.insert(
        "escapee".to_string(),
        renderer.load_texture("assets/textures/escapee.png")?,
    );

    Ok(textures)
}

unsafe fn load_skybox_textures(
    renderer: &mut VulkanRenderer,
) -> Result<HashMap<String, SkyboxTextureHandle>> {
    let mut textures = HashMap::new();
    textures.insert(
        "escapee".to_string(),
        renderer.load_skybox_texture([
            "assets/textures/escapee.png",
            "assets/textures/escapee.png",
            "assets/textures/escapee.png",
            "assets/textures/escapee.png",
            "assets/textures/escapee.png",
            "assets/textures/escapee.png",
        ])?,
    );
    textures.insert(
        "ghost".to_string(),
        renderer.load_skybox_texture([
            "assets/textures/ghost.png",
            "assets/textures/ghost.png",
            "assets/textures/ghost.png",
            "assets/textures/ghost.png",
            "assets/textures/ghost.png",
            "assets/textures/ghost.png",
        ])?,
    );

    Ok(textures)
}

unsafe fn create_skybox_mesh(renderer: &mut VulkanRenderer, size: f32) -> Result<MeshHandle> {
    let h = size;
    let source = build_primitive_source(PrimitiveShape::Cube {
        points: [
            vec3(h, -h, h),
            vec3(h, h, h),
            vec3(-h, h, h),
            vec3(-h, -h, h),
            vec3(h, -h, -h),
            vec3(h, h, -h),
            vec3(-h, h, -h),
            vec3(-h, -h, -h),
        ],
        color: vec3(1.0, 1.0, 1.0),
    });

    renderer.load_mesh_from_data(source.to_skybox_data(), VertexLayout::Skybox)
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
    use crate::KeyBinding;
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
                Some(Camera {
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

        assert_eq!(world.registry.entity_count(), ids.len());
        assert!(ids.iter().all(|id| world.registry.contains(*id)));
    }

    #[test]
    fn register_and_reset_input_commands() {
        let mut input_system = InputSystem::new()
            .bind(
                KeyCode::ArrowLeft,
                InputTrigger::Pressed,
                InputCommand::DespawnLast,
            )
            .bind(
                KeyCode::ArrowRight,
                InputTrigger::Pressed,
                InputCommand::SpawnVikingRoom,
            )
            .bind(
                KeyCode::ArrowRight,
                InputTrigger::Pressed,
                InputCommand::SpawnVikingRoom,
            )
            .bind(
                KeyCode::KeyT,
                InputTrigger::Pressed,
                InputCommand::SpawnPrimitive {
                    primitive_type: PrimitiveType::Triangle,
                    pipeline_key: PipelineKey::Lit3D,
                    texture_name: Some("face"),
                },
            );

        // check not to register same command
        assert_eq!(input_system.key_bindings.len(), 3);
        assert!(input_system.key_bindings.contains(&KeyBinding {
            key: KeyCode::ArrowRight,
            trigger: InputTrigger::Pressed,
            command: InputCommand::SpawnVikingRoom,
        }));

        input_system.reset();
        assert!(input_system.key_bindings.is_empty());
    }
}
