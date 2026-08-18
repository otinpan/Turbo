#![allow(
    dead_code,
    unsafe_op_in_unsafe_fn,
    unused_variables,
    clippy::too_many_arguments,
    clippy::unnecessary_wraps
)]

mod debug_monitor;
mod despawn_last_command;
mod spawn_primitive_command;
mod spawn_viking_room_command;
mod update_primitive_meshes_command;

pub use debug_monitor::DebugMonitor;
pub use despawn_last_command::DespawnLastCommand;
pub use spawn_primitive_command::SpawnPrimitiveCommand;
pub use spawn_viking_room_command::SpawnVikingRoomCommand;
pub use update_primitive_meshes_command::UpdatePrimitiveMeshesCommand;

use anyhow::Result;
use cgmath::Vector3;

use crate::{
    CommandQueue, Component, ComponentPool, EntityId, Input, RenderCommandQueue, Resources, World,
};

pub type Vec3 = Vector3<f32>;

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
}

pub trait Command {
    fn id(&self) -> String;
    fn execute(&self, context: &mut CommandContext<'_>) -> Result<()>;
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
