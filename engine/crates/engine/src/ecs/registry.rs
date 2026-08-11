use turbo_math::Transform;

use super::{
    ComponentPool, EntityId,
};

use crate::component::{
    Camera, Material, MeshRenderer, Rotator, Visibility,
};


pub type Vec3 = cgmath::Vector3<f32>;

pub trait RegistryComponent: Sized {
    fn pool(registry: &Registry) -> &ComponentPool<Self>;
    fn pool_mut(registry: &mut Registry) -> &mut ComponentPool<Self>;
}


// components for rendering
#[derive(Clone, Debug)]
pub struct RenderableRef<'a> {
    pub entity: EntityId,
    pub transform: &'a Transform,
    pub mesh_renderer: &'a MeshRenderer,
    pub visibility: Option<&'a Visibility>,
}

#[derive(Clone, Debug)]
pub struct Registry{
    next_entity_id: usize,
    entities: Vec<EntityId>,

    // component
    transform: ComponentPool<Transform>,
    camera: ComponentPool<Camera>,
    material: ComponentPool<Material>,
    mesh_renderer: ComponentPool<MeshRenderer>,
    rotator: ComponentPool<Rotator>,
    visibility: ComponentPool<Visibility>,
}

impl Registry{
    pub fn create(&mut self) -> EntityId{
        let entity=EntityId(self.next_entity_id);
        self.next_entity_id+=1;
        self.entities.push(entity);
        entity
    }
    pub fn contains(&self, entity: EntityId) -> bool {
        self.entities.contains(&entity)
    }

    pub fn entity_count(&self) -> usize {
        self.entities.len()
    }


    // Component access ////////////////////////////////////////////
    pub fn add_component<T: RegistryComponent>(&mut self, entity: EntityId, component: T) -> bool {
        if !self.contains(entity) {
            return false;
        }

        T::pool_mut(self).insert(entity, component);
        true
    }

    // remove component from entity, but not remove entity
    pub fn remove_component<T: RegistryComponent>(&mut self, entity: EntityId) -> Option<T> {
        T::pool_mut(self).remove(entity)
    }

    pub fn despawn(&mut self, entity: EntityId) -> bool {
        if !self.contains(entity) {
            return false;
        }

        self.transform.remove(entity);
        self.camera.remove(entity);
        self.material.remove(entity);
        self.mesh_renderer.remove(entity);
        self.rotator.remove(entity);
        self.visibility.remove(entity);

        self.entities.retain(|id| *id != entity);

        true
    }
    /*
    let Some(transform) = registry.get_component::<Transform>(entity) else {
        return;
    };
    */
    pub fn get_component<T: RegistryComponent>(&self, entity: EntityId) -> Option<&T> {
        T::pool(self).get(entity)
    }

    pub fn get_component_mut<T: RegistryComponent>(&mut self, entity: EntityId) -> Option<&mut T> {
        T::pool_mut(self).get_mut(entity)
    }

    pub fn has_component<T: RegistryComponent>(&self, entity: EntityId) -> bool {
        T::pool(self).contains(entity)
    }

    // Shortcuts ///////////////////////////////////////////////////
    pub fn transform(&self, entity: EntityId) -> Option<&Transform> {
        self.transform.get(entity)
    }

    pub fn transform_mut(&mut self, entity: EntityId) -> Option<&mut Transform> {
        self.transform.get_mut(entity)
    }

    pub fn mesh_renderer(&self, entity: EntityId) -> Option<&MeshRenderer> {
        self.mesh_renderer.get(entity)
    }

    pub fn mesh_renderer_mut(&mut self, entity: EntityId) -> Option<&mut MeshRenderer> {
        self.mesh_renderer.get_mut(entity)
    }

    pub fn camera(&self, entity: EntityId) -> Option<&Camera> {
        self.camera.get(entity)
    }

    pub fn camera_mut(&mut self, entity: EntityId) -> Option<&mut Camera> {
        self.camera.get_mut(entity)
    }

    pub fn visibility(&self, entity: EntityId) -> Option<&Visibility> {
        self.visibility.get(entity)
    }

    pub fn visibility_mut(&mut self, entity: EntityId) -> Option<&mut Visibility> {
        self.visibility.get_mut(entity)
    }

    pub fn rotator(&self, entity: EntityId) -> Option<&Rotator> {
        self.rotator.get(entity)
    }

    pub fn rotator_mut(&mut self, entity: EntityId) -> Option<&mut Rotator> {
        self.rotator.get_mut(entity)
    }

    // Queries /////////////////////////////////////////////////////
    pub fn entities(&self) -> &[EntityId] {
        &self.entities
    }

    pub fn renderables(&self) -> impl Iterator<Item = RenderableRef<'_>> {
        self.mesh_renderer
            .iter()
            .filter_map(|(entity, mesh_renderer)| {
                Some(RenderableRef {
                    entity,
                    transform: self.transform.get(entity)?,
                    mesh_renderer,
                    visibility: self.visibility.get(entity),
                })
            })
    }

    pub fn active_camera_entity(&self) -> Option<EntityId> {
        self.camera.iter().next().map(|(entity, _)| entity)
    }

}

impl Default for Registry{
    fn default() -> Self{
        Self {
            next_entity_id: 0,
            entities: Vec::new(),

            transform: ComponentPool::new(),
            camera: ComponentPool::new(),
            material: ComponentPool::new(),
            mesh_renderer: ComponentPool::new(),
            rotator: ComponentPool::new(),
            visibility: ComponentPool::new(),
        }
    }
}

impl RegistryComponent for Transform {
    fn pool(registry: &Registry) -> &ComponentPool<Self> {
        &registry.transform
    }

    fn pool_mut(registry: &mut Registry) -> &mut ComponentPool<Self> {
        &mut registry.transform
    }
}

impl RegistryComponent for Camera {
    fn pool(registry: &Registry) -> &ComponentPool<Self> {
        &registry.camera
    }

    fn pool_mut(registry: &mut Registry) -> &mut ComponentPool<Self> {
        &mut registry.camera
    }
}

impl RegistryComponent for Material {
    fn pool(registry: &Registry) -> &ComponentPool<Self> {
        &registry.material
    }

    fn pool_mut(registry: &mut Registry) -> &mut ComponentPool<Self> {
        &mut registry.material
    }
}

impl RegistryComponent for MeshRenderer {
    fn pool(registry: &Registry) -> &ComponentPool<Self> {
        &registry.mesh_renderer
    }

    fn pool_mut(registry: &mut Registry) -> &mut ComponentPool<Self> {
        &mut registry.mesh_renderer
    }
}

impl RegistryComponent for Rotator {
    fn pool(registry: &Registry) -> &ComponentPool<Self> {
        &registry.rotator
    }

    fn pool_mut(registry: &mut Registry) -> &mut ComponentPool<Self> {
        &mut registry.rotator
    }
}

impl RegistryComponent for Visibility {
    fn pool(registry: &Registry) -> &ComponentPool<Self> {
        &registry.visibility
    }

    fn pool_mut(registry: &mut Registry) -> &mut ComponentPool<Self> {
        &mut registry.visibility
    }
}

// test ///////////////////////////////////////////
#[cfg(test)]
mod tests {
    use super::*;
    use crate::Material;
    use cgmath::vec3;
    use renderer_vulkan::{MeshHandle, VertexLayout};

    fn mesh_handle(index: usize) -> MeshHandle {
        MeshHandle::new(index, VertexLayout::Mesh3D)
    }

    fn spawn_renderable(
        registry: &mut Registry,
        mesh: MeshHandle,
        transform: Transform,
    ) -> EntityId {
        let entity = registry.create();
        registry.add_component(entity, transform);
        registry.add_component(
            entity,
            MeshRenderer {
                mesh,
                material: Material::default(),
            },
        );
        registry.add_component(
            entity,
            Rotator {
                speed: vec3(20.0, 0.0, 0.0),
            },
        );
        registry.add_component(entity, Visibility::default());

        entity
    }

    #[test]
    fn create_adds_entity_and_returns_unique_entity_ids() {
        let mut registry = Registry::default();
        let mesh = mesh_handle(0);

        let first = spawn_renderable(&mut registry, mesh, Transform::default());
        let second = spawn_renderable(&mut registry, mesh, Transform::default());

        assert_eq!(registry.entity_count(), 2);
        assert_ne!(first, second);
        assert_eq!(first, EntityId(0));
        assert_eq!(second, EntityId(1));
    }

    #[test]
    fn shortcuts_return_components_for_entity() {
        let mut registry = Registry::default();
        let mesh = mesh_handle(7);
        let transform = Transform {
            position: vec3(0.0, 1.0, 1.0),
            rotation: vec3(-1.0, 2.0, 3.0),
            scale: vec3(1.0, 2.0, 3.0),
        };

        let entity = spawn_renderable(&mut registry, mesh, transform.clone());

        assert_eq!(registry.mesh_renderer(entity).unwrap().mesh, mesh);
        assert_eq!(
            registry.transform(entity).unwrap().position,
            transform.position
        );
        assert_eq!(
            registry.transform(entity).unwrap().rotation,
            transform.rotation
        );
        assert_eq!(registry.transform(entity).unwrap().scale, transform.scale);
    }

    #[test]
    fn transform_mut_can_change_transform() {
        let mut registry = Registry::default();
        let entity = spawn_renderable(&mut registry, mesh_handle(0), Transform::default());

        registry.transform_mut(entity).unwrap().position = vec3(1.0, 2.0, 3.0);

        assert_eq!(
            registry.transform(entity).unwrap().position,
            vec3(1.0, 2.0, 3.0)
        );
    }

    #[test]
    fn despawn_removes_entity_and_components() {
        let mut registry = Registry::default();
        let first = spawn_renderable(&mut registry, mesh_handle(0), Transform::default());
        let second = spawn_renderable(&mut registry, mesh_handle(1), Transform::default());

        assert!(registry.despawn(first));

        assert_eq!(registry.entity_count(), 1);
        assert!(!registry.contains(first));
        assert!(registry.contains(second));
        assert!(registry.transform(first).is_none());
        assert!(registry.mesh_renderer(first).is_none());
    }

    #[test]
    fn despawn_unknown_entity_returns_false() {
        let mut registry = Registry::default();

        assert!(!registry.despawn(EntityId(999)));
        assert_eq!(registry.entity_count(), 0);
    }

    #[test]
    fn component_access_can_add_get_and_remove_components() {
        let mut registry = Registry::default();
        let entity = registry.create();
        registry.add_component(entity, Transform::default());

        assert!(registry.add_component(
            entity,
            Rotator {
                speed: vec3(1.0, 2.0, 3.0),
            }
        ));
        assert!(registry.has_component::<Rotator>(entity));
        assert_eq!(registry.get_component::<Rotator>(entity).unwrap().speed.x, 1.0);
        assert!(registry.remove_component::<Rotator>(entity).is_some());
        assert!(!registry.has_component::<Rotator>(entity));
    }

    #[test]
    fn renderables_iterates_mesh_renderer_entities_with_transform() {
        let mut registry = Registry::default();
        let renderable = spawn_renderable(&mut registry, mesh_handle(0), Transform::default());
        let entity = registry.create();
        registry.add_component(entity, Transform::default());

        let renderables = registry.renderables().collect::<Vec<_>>();

        assert_eq!(renderables.len(), 1);
        assert_eq!(renderables[0].entity, renderable);
    }

}
