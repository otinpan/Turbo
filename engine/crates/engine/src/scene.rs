use cgmath::{Vector3};
use crate::system::{
    AssetApi, CommandContext, EntityApi, ObjectApi, RenderCommandApi, UpdateContext,
    Command, InputTrigger, UpdateSystem,
};
use crate::component::{
    Visibility,
};
use crate::{
    Input, RenderCommandQueue, Resources, Scheduler, Time, World,
    MeshAssetId, Material, EntityId, PendingPrimitiveMesh,
};
use crate::primitive::{
    spawn_primitive_from_mesh, PrimitiveShape,
};
use renderer_vulkan::{
    PipelineKey,
};
use crate::app::{DEFAULT_TEXTURE};

use turbo_math::{Transform};
use anyhow::Result;

type Vec3=Vector3<f32>;

pub struct SceneContext<'a> {
    world: &'a mut World,
    input: &'a Input,
    time: &'a Time,
    // resource
    resources: &'a mut Resources,
    render_commands: &'a mut RenderCommandQueue,
    // system
    scheduler: &'a mut Scheduler,
}

impl<'a> SceneContext<'a> {
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

    // bind Command with (key,trigger)
    pub fn bind_input_command<C>(
        &mut self,
        key: winit::keyboard::KeyCode,
        trigger: InputTrigger,
        command: C
    )
    where
        C: Command + 'static,
    {
        self.scheduler.bind_key(key,trigger,command);
    }

    // insert UpdateSystem to Scheduler
    pub fn add_update_system<S>(&mut self, name: &str, system: S)
    where
        S: UpdateSystem + 'static,
    {
        self.scheduler.add_update_system(name,system);
    }

}

impl EntityApi for SceneContext<'_> {
    fn world(&self) -> &World {
        &self.world
    }
    fn world_mut(&mut self) -> &mut World {
        &mut self.world
    }

    fn resources(&self) -> &Resources {
        &self.resources
    }
    fn resources_mut(&mut self) -> &mut Resources {
        &mut self.resources
    }

    fn render_commands(&self) -> &RenderCommandQueue {
        &self.render_commands
    }
    fn render_commands_mut(&mut self) -> &mut RenderCommandQueue {
        &mut self.render_commands
    }
}

impl AssetApi for SceneContext<'_>{
    fn resources(&self) -> &Resources{
        &self.resources
    }
    fn resources_mut(&mut self) -> &mut Resources{
        &mut self.resources
    }
}

impl ObjectApi for SceneContext<'_>{
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

impl RenderCommandApi for SceneContext<'_>{
    fn render_commands_mut(&mut self) -> &mut RenderCommandQueue{
        &mut self.render_commands
    }
}

pub trait Scene {
    fn on_enter(&mut self, context: &mut SceneContext<'_>) -> Result<()>;

    fn update(&mut self, context: &mut UpdateContext<'_>) -> Result<()>;

    fn on_exit(&mut self, context: &mut CommandContext<'_>) -> Result<()>;
}
