use crate::{
    Component, ComponentPool, EntityId, MeshRenderer, RenderCommandQueue, Resources, World,
};

pub trait EntityApi {
    fn world(&self) -> &World;
    fn world_mut(&mut self) -> &mut World;

    fn render_commands(&self) -> &RenderCommandQueue;
    fn render_commands_mut(&mut self) -> &mut RenderCommandQueue;

    fn resources(&self) -> &Resources;
    fn resources_mut(&mut self) -> &mut Resources;

    fn spawn(&mut self) -> EntityId {
        self.world_mut().spawn()
    }

    fn despawn(&mut self, entity: EntityId) -> bool {
        if !self.world_mut().contains(entity) {
            return false;
        }

        let asset_id = self
            .world()
            .get_component::<MeshRenderer>(entity)
            .and_then(|mesh_renderer| mesh_renderer.asset_id);

        if let Some(asset_id) = asset_id {
            if let Some(mesh) = self.resources_mut().release_mesh(asset_id) {
                self.render_commands_mut().destroy_mesh(mesh);
            }
        }

        self.world_mut().despawn(entity)
    }

    fn despawn_last(&mut self) -> bool {
        let Some(entity) = self.entities().last().copied() else {
            return false;
        };

        self.despawn(entity)
    }

    fn entities(&self) -> &[EntityId] {
        self.world().entities()
    }

    fn is_entity_registered(&self, entity: EntityId) -> bool {
        self.world().contains(entity)
    }

    fn entity_count(&self) -> usize {
        self.world().entity_count()
    }

    fn add_component<T: Component>(&mut self, entity: EntityId, component: T) -> bool {
        self.world_mut().add_component::<T>(entity, component)
    }

    fn remove_component<T: Component>(&mut self, entity: EntityId, component: T) -> Option<T> {
        self.world_mut().remove_component::<T>(entity)
    }

    fn get_component_pool<T: Component>(&self) -> Option<&ComponentPool<T>> {
        self.world().get_pool::<T>()
    }

    fn get_component_pool_mut<T: Component>(&mut self) -> Option<&mut ComponentPool<T>> {
        self.world_mut().get_pool_mut::<T>()
    }

    fn get_component<T: Component>(&self, entity: EntityId) -> Option<&T> {
        self.world().get_component::<T>(entity)
    }

    fn get_component_mut<T: Component>(&mut self, entity: EntityId) -> Option<&mut T> {
        self.world_mut().get_component_mut::<T>(entity)
    }

    fn has_component<T: Component>(&self, entity: EntityId) -> bool {
        self.world().has_component::<T>(entity)
    }

    fn query2<A, B>(&self) -> Box<dyn Iterator<Item = (EntityId, &A, &B)> + '_>
    where
        A: Component,
        B: Component,
    {
        self.world().query2::<A, B>()
    }

    fn query2_mut<A, B>(&mut self) -> Box<dyn Iterator<Item = (EntityId, &mut A, &B)> + '_>
    where
        A: Component,
        B: Component,
    {
        self.world_mut().query2_mut::<A, B>()
    }

    fn query2_mut_mut<A, B>(&mut self) -> Box<dyn Iterator<Item = (EntityId, &mut A, &mut B)> + '_>
    where
        A: Component,
        B: Component,
    {
        self.world_mut().query2_mut_mut::<A, B>()
    }

    fn find_entity_by_name(&self, name: &str) -> Option<EntityId> {
        self.world().find_by_name(name)
    }

    fn set_name(&mut self, entity: EntityId, name: &str) -> bool {
        self.world_mut().set_name(entity, name)
    }

    fn remove_name(&mut self, entity: EntityId) -> bool {
        self.world_mut().remove_name(entity)
    }

    fn set_tags<const N: usize>(&mut self, entity: EntityId, tags: [&str; N]) -> bool {
        self.world_mut().set_tags(entity, tags)
    }

    fn remove_tags(&mut self, entity: EntityId) -> bool {
        self.world_mut().remove_tags(entity)
    }

    fn remove_tag(&mut self, entity: EntityId, tag: &str) -> bool {
        self.world_mut().remove_tag(entity, tag)
    }
    fn get_entities_by_tag(&self, tag: &str) -> Vec<EntityId> {
        self.world().find_by_tag(tag)
    }

    fn get_all_named_entities(&self) -> Vec<(String, EntityId)> {
        self.world().get_all_named_entities()
    }

    fn get_all_taged_entities(&self) -> Vec<(String, EntityId)> {
        self.world().get_all_taged_entities()
    }
}
