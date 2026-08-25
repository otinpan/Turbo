#![allow(
    dead_code,
    unsafe_op_in_unsafe_fn,
    unused_variables,
    clippy::too_many_arguments,
    clippy::unnecessary_wraps
)]
use anyhow::{Result,anyhow};
use cgmath::{Vector2,Vector3};

mod camera_system;
mod rotator_system;

pub use super::render_command::RenderCommandQueue;
use crate::{
    ComponentPool, EntityId, Input, Resources, Time, World, 
};
use turbo_math::{Transform};
use crate::component::{
    Component, Material, MeshRenderer, Visibility
};
pub use camera_system::CameraSystem;
pub use rotator_system::RotatorSystem;

type Vec2=Vector2<f32>;
type Vec3=Vector3<f32>;

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

    pub fn input(&self) -> &Input {
        self.input
    }

    pub fn delta_seconds(&self) -> f32 {
        self.time.delta_seconds()
    }

    pub fn spawn(&mut self) -> EntityId {
        self.world.spawn()
    }

    pub fn despawn(&mut self, entity: EntityId) -> bool {
        self.world.despawn(entity)
    }

    pub fn entities(&self) -> &[EntityId] {
        self.world.entities()
    }

    pub fn is_entity_registered(&self, entity: EntityId) -> bool {
        self.world.contains(entity)
    }

    pub fn entity_count(&self) -> usize {
        self.world.entity_count()
    }

    pub fn add_component<T: Component>(&mut self, entity: EntityId, component: T) -> bool {
        self.world.add_component::<T>(entity, component)
    }

    pub fn remove_component<T: Component>(&mut self, entity: EntityId, component: T) -> Option<T> {
        self.world.remove_component::<T>(entity)
    }

    pub fn get_component_pool<T: Component>(&self) -> Option<&ComponentPool<T>> {
        self.world.get_pool::<T>()
    }

    pub fn get_component_pool_mut<T: Component>(&mut self) -> Option<&mut ComponentPool<T>> {
        self.world.get_pool_mut::<T>()
    }

    pub fn get_component<T: Component>(&mut self, entity: EntityId) -> Option<&T> {
        self.world.get_component::<T>(entity)
    }

    pub fn get_component_mut<T: Component>(&mut self, entity: EntityId) -> Option<&mut T> {
        self.world.get_component_mut::<T>(entity)
    }

    pub fn has_component<T: Component>(&self, entity: EntityId) -> bool {
        self.world.has_component::<T>(entity)
    }

    pub fn query2<A, B>(&self) -> Box<dyn Iterator<Item = (EntityId, &A, &B)> + '_>
    where
        A: Component,
        B: Component,
    {
        self.world.query2::<A, B>()
    }

    pub fn query2_mut<A, B>(&mut self) -> Box<dyn Iterator<Item = (EntityId, &mut A, &B)> + '_>
    where
        A: Component,
        B: Component,
    {
        self.world.query2_mut::<A, B>()
    }

    pub fn query2_mut_mut<A, B>(
        &mut self,
    ) -> Box<dyn Iterator<Item = (EntityId, &mut A, &mut B)> + '_>
    where
        A: Component,
        B: Component,
    {
        self.world.query2_mut_mut::<A, B>()
    }

    pub fn find_entity_by_name(&self, name: &str) -> Option<EntityId> {
        self.world.find_by_name(name)
    }

    pub fn set_name(&mut self, entity: EntityId, name: &str) -> bool {
        self.world.set_name(entity, name)
    }

    pub fn remove_name(&mut self, entity: EntityId) -> bool {
        self.world.remove_name(entity)
    }

    pub fn set_tags<const N: usize>(&mut self, entity: EntityId, tags: [&str; N]) -> bool {
        self.world.set_tags(entity, tags)
    }

    pub fn remove_tags(&mut self, entity: EntityId) -> bool {
        self.world.remove_tags(entity)
    }

    pub fn remove_tag(&mut self, entity: EntityId, tag: &str) -> bool {
        self.world.remove_tag(entity, tag)
    }

    pub fn get_entities_by_tag(&self, tag: &str) -> Vec<EntityId> {
        self.world.find_by_tag(tag)
    }

    pub fn get_all_named_entities(&self) -> Vec<(String, EntityId)> {
        self.world.get_all_named_entities()
    }

    pub fn get_all_taged_entities(&self) -> Vec<(String, EntityId)> {
        self.world.get_all_taged_entities()
    }

    pub fn mouse_position(&self) -> Vec2{
        self.input.mouse_position()
    }

    pub fn window_size(&self) -> Vec2{
        self.input.window_size()
    }

    pub fn spawn_model(
        &mut self,
        model_name: &str,
        transform: Transform,
        material: Material,
    ) -> Result<EntityId>{
        let asset_id=self
            .resources
            .model_asset_id(model_name)
            .ok_or_else(||anyhow!("model not found: {model_name}"))?;

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

}

// user api

pub struct ScheduledUpdateSystem {
    pub name: String,
    pub system: Box<dyn UpdateSystem>,
    pub enabled: bool,
}
pub trait UpdateSystem {
    fn update(&mut self, context: &mut UpdateContext<'_>) -> Result<()>;
}
