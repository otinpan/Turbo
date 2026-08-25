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
    spawn_line_with_material, spawn_polygon_with_material, spawn_rectangle_with_material,
    spawn_sphere_with_material, spawn_triangle_with_material,
};

use super::{
    Camera, CameraSystem, Command, CommandContext, EntityId, Input, InputTrigger, Material,
    MeshRenderer, ObjectApi, Registry, Resources, RotatorSystem, Scheduler, Time, Visibility,
    World,
};

use super::system::{
    CreatePrimitiveCommand, DebugMonitor, DespawnLastCommand, SpawnPrimitiveCommand,
    SpawnVikingRoomCommand, UpdatePrimitiveMeshesCommand,
};

pub type Vec3 = cgmath::Vector3<f32>;
pub type Vec2 = cgmath::Vector2<f32>;

pub struct App {
    pub renderer: VulkanRenderer,
    pub world: World,
    pub input: Input,
    pub time: Time,
    // resource
    resources: Resources,

    // system
    scheduler: Scheduler,

    positions: Vec<Vec3>,
}

impl App {
    pub unsafe fn create(window: &Window) -> Result<Self> {
        let mut renderer = VulkanRenderer::create(window)?;
        let world = create_world();

        // load data
        let mut resources = Resources::default();
        load_models(&mut renderer, &mut resources)?;
        let primitive_meshes = create_primitive_meshes(&mut renderer, &mut resources)?;
        resources.set_primitive_meshes(primitive_meshes);
        resources.set_textures(load_textures(&mut renderer)?);
        resources.set_skybox_mesh(create_skybox_mesh(&mut renderer, 20.0)?);
        resources.set_skybox_textures(load_skybox_textures(&mut renderer)?);

        let positions = vec![
            vec3(0.0, -1.25, 1.0),
            vec3(0.0, 1.25, 1.0),
            vec3(0.0, -1.25, -1.0),
            vec3(0.0, 1.25, -1.0),
        ];

        let mut input = Input::default();
        let window_size = window.inner_size();
        input.set_window_size(vec2(window_size.width as f32, window_size.height as f32));

        let scheduler = create_scheduler();

        let mut app = Self {
            renderer,
            world,
            input,
            time: Time::default(),
            resources,
            positions,
            scheduler,
        };

        #[cfg(debug_assertions)]
        {
            app.set_skybox(app.use_skybox_texture("ghost"))?;
            // create primitive ////////////////////////////
            unsafe {
                let face_texture = app.use_texture("face");
                let ghost_texture = app.use_texture("ghost");
                let escapee_texture = app.use_texture("escapee");
                let viking_room = app.spawn_model(
                    "viking_room_lit3d",
                    Transform {
                        position: vec3(-5.0, 0.0, 2.0),
                        ..Default::default()
                    },
                    Material {
                        color: vec3(1.0, 1.0, 0.0),
                        alpha: 1.0,
                        use_texture: false,
                        texture: ghost_texture,
                        pipeline_key: PipelineKey::Lit3D,
                    },
                );
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
            .run_render_stage(&mut app.world, &mut app.renderer, &mut app.resources)?;

        Ok(app)
    }

    pub fn handle_event(&mut self, event: &WindowEvent) {
        self.input.handle_event(event);
    }

    pub unsafe fn render(&mut self, window: &Window) -> Result<()> {
        self.renderer.render(window)
    }

    pub fn despawn(&mut self, entity: EntityId) -> Result<bool> {
        if let Some(mesh_renderer) = self.world.get_component::<MeshRenderer>(entity) {
            if let Some(asset_id) = mesh_renderer.asset_id {
                if let Some(mesh) = self.resources.release_mesh(asset_id) {
                    self.scheduler.render_commands.destroy_mesh(mesh);
                }
            }
        }

        Ok(self.world.despawn(entity))
    }

    pub fn update(&mut self) -> Result<()> {
        self.time.update();
        let delta_time = self.time.delta_seconds();

        let mut commands = self.scheduler.run_input_stage(&self.input);

        self.scheduler.run_command_stage(
            &mut commands,
            &mut self.world,
            &self.input,
            &mut self.resources,
            &self.positions,
        )?;

        self.scheduler.run_update_stage(
            &mut self.world,
            &self.input,
            &self.time,
            &mut self.resources,
        )?;

        self.scheduler.run_render_stage(
            &mut self.world,
            &mut self.renderer,
            &mut self.resources,
        )?;

        self.input.clear_transitions();
        Ok(())
    }

    fn bind_input_commands(scheduler: &mut Scheduler) {
        scheduler.bind_key(
            KeyCode::ArrowLeft,
            InputTrigger::Pressed,
            DespawnLastCommand,
        );
        scheduler.bind_key(
            KeyCode::ArrowRight,
            InputTrigger::Pressed,
            SpawnVikingRoomCommand,
        );
        scheduler.bind_key(
            KeyCode::KeyT,
            InputTrigger::Pressed,
            SpawnPrimitiveCommand {
                primitive_type: PrimitiveType::Triangle,
                pipeline_key: PipelineKey::Lit3D,
                texture_name: Some("face"),
            },
        );
        scheduler.bind_key(
            KeyCode::KeyR,
            InputTrigger::Pressed,
            SpawnPrimitiveCommand {
                primitive_type: PrimitiveType::Rectangle,
                pipeline_key: PipelineKey::Ui2D,
                texture_name: Some("face"),
            },
        );
        scheduler.bind_key(
            KeyCode::KeyC,
            InputTrigger::Pressed,
            SpawnPrimitiveCommand {
                primitive_type: PrimitiveType::Cube,
                pipeline_key: PipelineKey::DebugLine3D,
                texture_name: None,
            },
        );
        scheduler.bind_key(
            KeyCode::KeyI,
            InputTrigger::Pressed,
            SpawnPrimitiveCommand {
                primitive_type: PrimitiveType::Circle,
                pipeline_key: PipelineKey::Mesh3D,
                texture_name: None,
            },
        );
        scheduler.bind_key(
            KeyCode::KeyP,
            InputTrigger::Pressed,
            SpawnPrimitiveCommand {
                primitive_type: PrimitiveType::Polygon,
                pipeline_key: PipelineKey::Mesh3D,
                texture_name: None,
            },
        );
        scheduler.bind_key(
            KeyCode::KeyE,
            InputTrigger::Pressed,
            SpawnPrimitiveCommand {
                primitive_type: PrimitiveType::Sphere,
                pipeline_key: PipelineKey::Lit3D,
                texture_name: None,
            },
        );
        scheduler.bind_key(
            KeyCode::KeyU,
            InputTrigger::Pressed,
            UpdatePrimitiveMeshesCommand,
        );

        // create primitives
        struct CreateTriangle {
            p0: Vec3,
            p1: Vec3,
            p2: Vec3,
            color: Vec3,
            alpha: f32,
            texture: Option<&'static str>,
            pipeline_key: PipelineKey,
        }
        impl Command for CreateTriangle {
            fn id(&self) -> String {
                format!("create_triangle")
            }
            fn execute(&self, context: &mut CommandContext<'_>) -> Result<()> {
                context.spawn_triangle_3d(
                    self.p0,
                    self.p1,
                    self.p2,
                    self.color,
                    self.alpha,
                    self.texture,
                    self.pipeline_key,
                )?;

                Ok(())
            }
        }
        scheduler.bind_key(
            KeyCode::Digit1,
            InputTrigger::Pressed,
            CreateTriangle {
                p0: vec3(0.0, 2.0, -0.3),
                p1: vec3(-7.0, 2.0, 0.3),
                p2: vec3(-2.0, 2.0, 1.0),
                color: vec3(1.0, 1.0, 0.0),
                alpha: 1.0,
                texture: Some("face"),
                pipeline_key: PipelineKey::Lit3D,
            },
        );
        scheduler.bind_key(
            KeyCode::Digit2,
            InputTrigger::Pressed,
            CreatePrimitiveCommand {
                primitive_shape: PrimitiveShape::Triangle {
                    points: [
                        vec3(0.0, 2.0, -0.3),
                        vec3(-7.0, 2.0, 0.3),
                        vec3(-2.0, 2.0, 1.0),
                    ],
                    color: vec3(1.0, 1.0, 1.0),
                },
                transform: Transform {
                    position: vec3(-5.0, 2.0, 0.0),
                    ..Default::default()
                },
                color: vec3(1.0, 1.0, 0.0),
                alpha: 1.0,
                texture: Some("face"),
                pipeline_key: PipelineKey::DebugLine3D,
                auto_release: true,
            },
        );
        scheduler.bind_key(KeyCode::Enter, InputTrigger::Pressed, DebugMonitor)
    }

    fn use_skybox_texture(&self, name: &str) -> SkyboxTextureHandle {
        self.resources
            .skybox_texture(name)
            .unwrap_or(DEFAULT_SKYBOX_TEXTURE)
    }

    fn use_texture(&self, name: &str) -> TextureHandle {
        self.resources
            .get_texture_handle(name)
            .unwrap_or(DEFAULT_TEXTURE)
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
            &mut self.resources,
            p0,
            p1,
            p2,
            material,
            true,
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
            &mut self.resources,
            vec3(0.0, p0.x, p0.y),
            vec3(0.0, p1.x, p1.y),
            vec3(0.0, p2.x, p2.y),
            material,
            true,
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
            &mut self.resources,
            pos,
            width,
            height,
            rotation,
            material,
            true,
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
            &mut self.resources,
            vec3(0.0, pos.x, pos.y),
            width,
            height,
            vec3(rotation, 0.0, 0.0),
            material,
            true,
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
            &mut self.resources,
            pos,
            length,
            rotation,
            material,
            true,
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
            &mut self.resources,
            pos,
            radius,
            segments,
            material,
            true,
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
            &mut self.resources,
            vec3(0.0, pos.x, pos.y),
            radius,
            segments,
            material,
            true,
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
            &mut self.resources,
            points,
            material,
            true,
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
            &mut self.resources,
            points,
            material,
            true,
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
            &mut self.resources,
            center,
            radius,
            rings,
            segments,
            material,
            true,
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
            &mut self.resources,
            pos0,
            pos1,
            material,
            true,
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
            &mut self.resources,
            center,
            width,
            length,
            rotation,
            material,
            true,
        )
    }

    pub unsafe fn set_skybox(&mut self, texture: SkyboxTextureHandle) -> Result<()> {
        let mesh = self
            .resources
            .skybox_mesh()
            .ok_or_else(|| anyhow!("Skybox mesh has not been created."))?;

        self.renderer.set_skybox(mesh, texture)
    }

    pub fn spawn_model(
        &mut self,
        model_name: &str,
        transform: Transform,
        material: Material,
    ) -> Result<EntityId> {
        let asset_id = self
            .resources
            .model_asset_id(model_name)
            .ok_or_else(|| anyhow!("model not found: {model_name}"))?;
        let mesh = self
            .resources
            .retain_mesh(asset_id)
            .ok_or_else(|| anyhow!("mesh asset not found: {asset_id:?}"))?;
        let mesh_renderer = match MeshRenderer::new(mesh, material) {
            Ok(mesh_renderer) => mesh_renderer.with_asset_id(asset_id),
            Err(error) => {
                self.resources.release_mesh(asset_id);
                return Err(error);
            }
        };

        let entity = self.world.spawn();
        self.world.add_component(entity, transform);
        self.world.add_component(entity, mesh_renderer);
        self.world.add_component(entity, Visibility::default());
        self.world.set_tags(entity, ["Model", model_name]);

        Ok(entity)
    }
}

fn create_scheduler() -> Scheduler {
    let mut scheduler = Scheduler::default();
    // input command
    App::bind_input_commands(&mut scheduler);
    // update system
    scheduler.add_update_system("rotator", RotatorSystem);
    scheduler.add_update_system("camera", CameraSystem);

    scheduler
}

fn create_world() -> World {
    let mut registry = Registry::default();
    let camera = registry.create();
    registry.add_component(
        camera,
        Camera {
            target: vec3(0.0, 0.0, 0.0),
            fov_y: 45.0,
            near: 0.1,
            far: 100.0,
            yaw: std::f32::consts::PI,
            pitch: 0.0,
        },
    );
    registry.add_component(
        camera,
        Transform {
            position: vec3(-1.0, 0.0, 0.0),
            ..Default::default()
        },
    );

    let mut world = World::from_registry(registry);
    world.set_name(camera, "Camera");

    world
}

unsafe fn load_models(renderer: &mut VulkanRenderer, resources: &mut Resources) -> Result<()> {
    resources.register_model(
        "viking_room",
        renderer.load_mesh3d_from_model("assets/models/viking_room.obj")?,
        false,
    );
    resources.register_model(
        "viking_room_debug_line",
        renderer.load_debug_line_from_model("assets/models/viking_room.obj")?,
        false,
    );
    resources.register_model(
        "viking_room_lit3d",
        renderer.load_lit3d_from_model("assets/models/viking_room.obj")?,
        false,
    );

    Ok(())
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

// these primitives are not released (resources.release_mesh) by DespawnLastCommand
// becuase DespawnLastCommand release only Entitis that have MeshRenderer Component
unsafe fn create_primitive_meshes(
    renderer: &mut VulkanRenderer,
    resources: &mut Resources,
) -> Result<Vec<PrimitiveMesh>> {
    let primitive_meshes = vec![
        create_primitive_lit3d(
            renderer,
            resources,
            PrimitiveShape::Triangle {
                points: [
                    vec3(0.0, 0.0, 0.5),
                    vec3(0.0, -0.5, -0.5),
                    vec3(0.0, 0.5, -0.5),
                ],
                color: vec3(1.0, 1.0, 1.0),
            },
            false,
        )?,
        create_primitive_ui2d(
            renderer,
            resources,
            PrimitiveShape::Rectangle {
                points: [
                    vec3(0.0, -0.5, 0.5),
                    vec3(0.0, -0.5, -0.5),
                    vec3(0.0, 0.5, -0.5),
                    vec3(0.0, 0.5, 0.5),
                ],
                color: vec3(1.0, 1.0, 0.0),
            },
            false,
        )?,
        create_primitive_debug_line(
            renderer,
            resources,
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
            false,
        )?,
        create_primitive_mesh3d(
            renderer,
            resources,
            PrimitiveShape::Circle {
                radius: 1.0,
                segments: 32,
                color: vec3(0.0, 1.0, 1.0),
            },
            false,
        )?,
        create_primitive_mesh3d(
            renderer,
            resources,
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
            false,
        )?,
        create_primitive_lit3d(
            renderer,
            resources,
            PrimitiveShape::Sphere {
                radius: 1.0,
                rings: 32,
                segments: 32,
                color: vec3(0.0, 0.0, 1.0),
            },
            false,
        )?,
    ];

    Ok(primitive_meshes)
}

// test ///////////////////////////////////////////////////
#[cfg(test)]
mod tests {
    use super::*;
    use crate::KeyBinding;
    use crate::primitive::spawn_primitive_from_mesh;
    #[test]
    fn created_world_object_count_matches_created_entity_id_count() {
        let mut world = World::default();
        let mut resources = Resources::default();
        let triangle_mesh =
            resources.insert_mesh_asset(MeshHandle::new(0, VertexLayout::Mesh3D), false);
        let rectangle_mesh =
            resources.insert_mesh_asset(MeshHandle::new(1, VertexLayout::Mesh3D), false);
        let cube_mesh =
            resources.insert_mesh_asset(MeshHandle::new(2, VertexLayout::Mesh3D), false);

        let camera = world.spawn();
        world.add_component(camera, Transform::default());
        world.add_component(
            camera,
            Camera {
                target: vec3(0.0, 0.0, 0.0),
                fov_y: 45.0,
                near: 0.1,
                far: 100.0,
                yaw: 0.0,
                pitch: 0.0,
            },
        );

        let ids = vec![
            camera,
            spawn_primitive_from_mesh(
                &mut world,
                &mut resources,
                triangle_mesh,
                Material::default(),
                Transform::default(),
            )
            .unwrap(),
            spawn_primitive_from_mesh(
                &mut world,
                &mut resources,
                rectangle_mesh,
                Material::default(),
                Transform::default(),
            )
            .unwrap(),
            spawn_primitive_from_mesh(
                &mut world,
                &mut resources,
                cube_mesh,
                Material::default(),
                Transform::default(),
            )
            .unwrap(),
        ];

        assert_eq!(world.entity_count(), ids.len());
        assert!(ids.iter().all(|id| world.contains(*id)));
    }

    #[test]
    fn register_and_reset_input_commands() {
        let mut scheduler = Scheduler::default();
        scheduler.bind_key(
            KeyCode::ArrowLeft,
            InputTrigger::Pressed,
            DespawnLastCommand,
        );
        scheduler.bind_key(
            KeyCode::ArrowRight,
            InputTrigger::Pressed,
            SpawnVikingRoomCommand,
        );
        scheduler.bind_key(
            KeyCode::ArrowRight,
            InputTrigger::Pressed,
            SpawnVikingRoomCommand,
        );
        scheduler.bind_key(
            KeyCode::KeyT,
            InputTrigger::Pressed,
            SpawnPrimitiveCommand {
                primitive_type: PrimitiveType::Triangle,
                pipeline_key: PipelineKey::Lit3D,
                texture_name: Some("face"),
            },
        );

        // check not to register same command
        assert_eq!(scheduler.input_system.key_bindings.len(), 3);
        assert!(scheduler.input_system.key_bindings.contains(&KeyBinding {
            key: KeyCode::ArrowRight,
            trigger: InputTrigger::Pressed,
            command_id: "spawn_viking_room".to_string(),
            command: std::sync::Arc::new(SpawnVikingRoomCommand),
        }));

        scheduler.input_system.reset();
        assert!(scheduler.input_system.key_bindings.is_empty());
    }
}
