#![allow(
    dead_code,
    unsafe_op_in_unsafe_fn,
    unused_variables,
    clippy::too_many_arguments,
    clippy::unnecessary_wraps
)]
use anyhow::{Result, anyhow};
use cgmath::{Vector2, Vector3};

mod camera_system;
mod rotator_system;

pub use super::render_command::RenderCommandQueue;
use crate::app::DEFAULT_TEXTURE;
use crate::component::{Material, MeshRenderer, Visibility};
use crate::primitive::spawn_primitive_from_mesh;
use crate::{
    EntityId, Input, MeshAssetId, PendingPrimitiveMesh, PrimitiveShape, Resources, Time, World,
};
use renderer_vulkan::PipelineKey;
use turbo_math::Transform;

use crate::{AssetApi, EntityApi, InputApi, ObjectApi, RenderCommandApi};
pub use camera_system::CameraSystem;
pub use rotator_system::RotatorSystem;

type Vec2 = Vector2<f32>;
type Vec3 = Vector3<f32>;

pub struct UpdateContext<'a> {
    world: &'a mut World,
    input: &'a Input,
    time: &'a Time,
    resources: &'a mut Resources,
    render_commands: &'a mut RenderCommandQueue,
}

impl<'a> UpdateContext<'a> {
    pub(crate) fn new(
        world: &'a mut World,
        input: &'a Input,
        time: &'a Time,
        resources: &'a mut Resources,
        render_commands: &'a mut RenderCommandQueue,
    ) -> Self {
        Self {
            world,
            input,
            time,
            resources,
            render_commands,
        }
    }

    pub fn delta_seconds(&self) -> f32 {
        self.time.delta_seconds()
    }

    // create new primitive entity and new mesh
    // entity is create here, but mesh is created in RenderSystem using VulkanRenderer.
    // the frame this called do not render new primitive
    pub(crate) fn enqueue_spawn_shape(
        &mut self,
        shape: PrimitiveShape,
        transform: Transform,
        material: Material,
        auto_release: bool,
    ) -> Result<EntityId> {
        let entity = self.spawn();

        self.add_component(entity, transform);
        self.add_component(entity, Visibility::default());
        self.add_component(
            entity,
            PendingPrimitiveMesh {
                shape: shape.clone(),
                material,
                auto_release,
            },
        );

        self.render_commands.create_primitive_mesh(entity);

        Ok(entity)
    }
}

impl EntityApi for UpdateContext<'_> {
    fn world(&self) -> &World {
        &self.world
    }
    fn world_mut(&mut self) -> &mut World {
        &mut self.world
    }

    fn render_commands(&self) -> &RenderCommandQueue {
        &self.render_commands
    }
    fn render_commands_mut(&mut self) -> &mut RenderCommandQueue {
        &mut self.render_commands
    }

    fn resources(&self) -> &Resources {
        &self.resources
    }
    fn resources_mut(&mut self) -> &mut Resources {
        &mut self.resources
    }
}

impl ObjectApi for UpdateContext<'_> {
    fn spawn_model(
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

        let entity = self.spawn();

        self.add_component(entity, transform);
        self.add_component(entity, mesh_renderer);
        self.add_component(entity, Visibility::default());
        self.set_tags(entity, ["Model", model_name]);

        Ok(entity)
    }

    fn spawn_primitive_from_mesh(
        &mut self,
        asset_id: MeshAssetId,
        material: Material,
        transform: Transform,
    ) -> Result<EntityId> {
        spawn_primitive_from_mesh(self.world, self.resources, asset_id, material, transform)
    }

    fn primitive_material(
        &self,
        color: Vec3,
        alpha: f32,
        texture: Option<&str>,
        pipeline_key: PipelineKey,
    ) -> Result<Material> {
        let use_texture = texture.is_some();
        let texture = match texture {
            Some(texture_name) => self.texture(texture_name)?,
            None => DEFAULT_TEXTURE,
        };

        Ok(Material {
            color,
            alpha,
            use_texture,
            texture,
            pipeline_key,
        })
    }

    fn spawn_shape_with_material(
        &mut self,
        shape: PrimitiveShape,
        transform: Transform,
        material: Material,
        auto_release: bool,
    ) -> Result<EntityId> {
        self.enqueue_spawn_shape(shape, transform, material, auto_release)
    }
}

impl AssetApi for UpdateContext<'_> {
    fn resources(&self) -> &Resources {
        &self.resources
    }

    fn resources_mut(&mut self) -> &mut Resources {
        &mut self.resources
    }
}

impl InputApi for UpdateContext<'_> {
    fn input(&self) -> &Input {
        &self.input
    }
}

impl RenderCommandApi for UpdateContext<'_> {
    fn render_commands_mut(&mut self) -> &mut RenderCommandQueue {
        &mut self.render_commands
    }
}

pub struct ScheduledUpdateSystem {
    pub name: String,
    pub system: Box<dyn UpdateSystem>,
    pub enabled: bool,
}
pub trait UpdateSystem {
    fn update(&mut self, context: &mut UpdateContext<'_>) -> Result<()>;
}
