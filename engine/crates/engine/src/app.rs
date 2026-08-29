use anyhow::{Result, anyhow};
use cgmath::{vec2, vec3};
use renderer_vulkan::{
    MeshHandle, SkyboxTextureHandle, TextureHandle, VertexLayout, VulkanRenderer,
};
use winit::event::WindowEvent;
use winit::window::Window;

pub const DEFAULT_TEXTURE: TextureHandle = TextureHandle(0);
pub const DEFAULT_SKYBOX_TEXTURE: SkyboxTextureHandle = SkyboxTextureHandle(0);

use crate::primitive::{
    PrimitiveMesh, PrimitiveShape, build_primitive_source, create_primitive_debug_line,
    create_primitive_lit3d, create_primitive_mesh3d, create_primitive_ui2d,
};

use crate::{MeshAssetId, PipelineKey, Scene, SceneContext, SceneId, UpdateContext};

use super::{Input, Resources, SceneManager, Scheduler, Time, World};

pub type Vec3 = cgmath::Vector3<f32>;
pub type Vec2 = cgmath::Vector2<f32>;

pub struct App {
    pub renderer: VulkanRenderer,
    pub world: World,
    pub input: Input,
    pub time: Time,
    resources: Resources,

    // system
    scheduler: Scheduler,
    scene_manager: SceneManager,
}

impl App {
    pub unsafe fn create(window: &Window) -> Result<Self> {
        let mut renderer = VulkanRenderer::create(window)?;
        let world = World::default();

        // load data
        let mut resources = Resources::default();

        let primitive_meshes = create_primitive_meshes(&mut renderer, &mut resources)?;
        resources.set_primitive_meshes(primitive_meshes);
        resources.set_skybox_mesh(create_skybox_mesh(&mut renderer, 20.0)?);

        let mut input = Input::default();
        let window_size = window.inner_size();
        input.set_window_size(vec2(window_size.width as f32, window_size.height as f32));

        let scheduler = Scheduler::default();
        let scene_manager = SceneManager::default();
        let mut app = Self {
            renderer,
            world,
            input,
            time: Time::default(),
            resources,
            scheduler,
            scene_manager,
        };

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

    pub fn update(&mut self) -> Result<()> {
        self.time.update();
        let delta_time = self.time.delta_seconds();

        let mut commands = self.scheduler.run_input_stage(&self.input);

        self.scheduler.run_command_stage(
            &mut commands,
            &mut self.world,
            &self.input,
            &mut self.resources,
        )?;

        {
            let mut context = UpdateContext::new(
                &mut self.world,
                &self.input,
                &self.time,
                &mut self.resources,
                &mut self.scheduler.render_commands,
            );
            self.scene_manager.update_current_scene(&mut context)?;
        }

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

    pub unsafe fn destroy(&mut self) {
        self.renderer.destroy();
    }

    pub fn add_scene<S>(&mut self, scene: S) -> Result<SceneId>
    where
        S: Scene + 'static,
    {
        self.scene_manager.add_scene(scene)
    }

    pub fn set_current_scene(&mut self, name: &str) -> Result<SceneId> {
        if self.scene_manager.current_scene_name() == Some(name) {
            return Ok(self
                .scene_manager
                .current_scene_id()
                .expect("current scene should have a scene id"));
        }

        if let Some(current_scene_id) = self.scene_manager.current_scene_id() {
            let mut context = SceneContext::new(
                current_scene_id,
                &mut self.world,
                &self.input,
                &self.time,
                &mut self.resources,
                &mut self.scheduler,
            );

            self.scene_manager.exit_current_scene(&mut context)?;
        }

        let scene_id = self.scene_manager.set_current_scene(name)?;
        let mut context = SceneContext::new(
            scene_id,
            &mut self.world,
            &self.input,
            &self.time,
            &mut self.resources,
            &mut self.scheduler,
        );

        self.scene_manager.enter_current_scene(&mut context)?;

        Ok(scene_id)
    }

    pub unsafe fn load_model(
        &mut self,
        name: &str,
        path: &str,
        pipeline_key: PipelineKey,
        auto_release: bool,
    ) -> Result<MeshAssetId> {
        let vertex_layout = pipeline_key.required_vertex_layout();

        let handle = match vertex_layout {
            VertexLayout::Mesh3D => self.renderer.load_mesh3d_from_model(path)?,
            VertexLayout::DebugLine3D => self.renderer.load_debug_line_from_model(path)?,
            VertexLayout::Lit3D => self.renderer.load_lit3d_from_model(path)?,
            VertexLayout::Ui2D => {
                return Err(anyhow!("Ui2D model loading is not supported"));
            }
            VertexLayout::Skybox => {
                return Err(anyhow!("Skybox model loading is not supported"));
            }
        };

        Ok(self.resources.register_model(name, handle, auto_release))
    }

    pub unsafe fn load_texture(&mut self, name: &str, path: &str) -> Result<TextureHandle> {
        let handle = self.renderer.load_texture(path)?;

        Ok(self.resources.register_texture(name, handle))
    }

    pub unsafe fn load_skybox_texture(
        &mut self,
        name: &str,
        paths: [&str; 6],
    ) -> Result<SkyboxTextureHandle> {
        let handle = self.renderer.load_skybox_texture(paths)?;
        Ok(self.resources.register_skybox_texture(name, handle))
    }
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
