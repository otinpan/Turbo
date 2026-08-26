#![allow(
    dead_code,
    unsafe_op_in_unsafe_fn,
    unused_variables,
    clippy::too_many_arguments,
    clippy::unnecessary_wraps
)]

mod create_primitive_command;
mod debug_monitor;
mod despawn_last_command;
mod spawn_primitive_command;
mod spawn_viking_room_command;
mod update_primitive_meshes_command;

pub use create_primitive_command::CreatePrimitiveCommand;
pub use debug_monitor::DebugMonitor;
pub use despawn_last_command::DespawnLastCommand;
pub use spawn_primitive_command::SpawnPrimitiveCommand;
pub use spawn_viking_room_command::SpawnVikingRoomCommand;
pub use update_primitive_meshes_command::UpdatePrimitiveMeshesCommand;

use anyhow::{Result, anyhow};
use cgmath::Vector3;

use crate::app::{DEFAULT_TEXTURE};
use crate::component::{Material, MeshRenderer, PendingPrimitiveMesh, Visibility};
use crate::primitive::spawn_primitive_from_mesh;
use crate::{
    CommandQueue, EntityId, Input, MeshAssetId, PrimitiveShape,
    PrimitiveType, RenderCommandQueue, Resources, World,
};
use renderer_vulkan::{PipelineKey, VertexLayout};
use turbo_math::Transform;
use crate::{
    EntityApi, ObjectApi, AssetApi, InputApi,
};

pub type Vec3 = Vector3<f32>;
pub type Vec2 = cgmath::Vector2<f32>;

pub struct CommandContext<'a> {
    commands: &'a mut CommandQueue,
    world: &'a mut World,
    input: &'a Input,
    resources: &'a mut Resources,
    render_commands: &'a mut RenderCommandQueue,
    positions: &'a [Vec3],
}

impl<'a> CommandContext<'a> {
    pub(crate) fn new(
        commands: &'a mut CommandQueue,
        world: &'a mut World,
        input: &'a Input,
        resources: &'a mut Resources,
        render_commands: &'a mut RenderCommandQueue,
        positions: &'a [Vec3],
    ) -> Self {
        Self {
            commands,
            world,
            input,
            resources,
            render_commands,
            positions,
        }
    }

    pub fn positions(&self) -> &[Vec3] {
        self.positions
    }


    // resources /////////////////////////////////

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

    pub fn update_primitive_mesh(
        &mut self,
        primitive_type: PrimitiveType,
        vertex_layout: VertexLayout,
        shape: PrimitiveShape,
    ) {
        self.render_commands
            .update_primitive_mesh(primitive_type, vertex_layout, shape);
    }
}

pub trait Command {
    fn id(&self) -> String;
    fn execute(&self, context: &mut CommandContext<'_>) -> Result<()>;
}

impl ObjectApi for CommandContext<'_> {
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

impl EntityApi for CommandContext<'_> {
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

impl AssetApi for CommandContext<'_>{
    fn resources(&self) -> &Resources{
        &self.resources
    }

    fn resources_mut(&mut self) -> &mut Resources{
        &mut self.resources
    }
}

impl InputApi for CommandContext<'_>{
    fn input(&self) -> &Input{
        &self.input
    }
}

#[derive(Clone, Debug)]
pub struct CommandSystem;

impl CommandSystem {
    pub fn update(&self, context: &mut CommandContext<'_>) -> Result<()> {
        let commands = context.commands.drain(..).collect::<Vec<_>>();

        for command in commands {
            command.execute(context)?;
        }

        Ok(())
    }
}
