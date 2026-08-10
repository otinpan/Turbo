use anyhow::Result;
use cgmath::vec3;
use turbo_math::Transform;

use super::{Camera, ComponentPool, EntityId, Material, MeshRenderer, Rotator, Visibility};

pub type Vec3 = cgmath::Vector3<f32>;

pub trait WorldComponent: Sized {
    fn pool(world: &World) -> &ComponentPool<Self>;
    fn pool_mut(world: &mut World) -> &mut ComponentPool<Self>;
}


// components for rendering
#[derive(Clone, Debug)]
pub struct RenderableRef<'a> {
    pub entity: EntityId,
    pub transform: &'a Transform,
    pub mesh_renderer: &'a MeshRenderer,
    pub visibility: Option<&'a Visibility>,
}

// World ///////////////////////////////////////////
#[derive(Clone, Debug)]
pub struct World {
    next_entity_id: usize,
    entities: Vec<EntityId>,

    transform: ComponentPool<Transform>,
    camera: ComponentPool<Camera>,
    material: ComponentPool<Material>,
    mesh_renderer: ComponentPool<MeshRenderer>,
    rotator: ComponentPool<Rotator>,
    visibility: ComponentPool<Visibility>,
}

impl World {
    pub fn spawn(
        &mut self,
        transform: Transform,
        mesh_renderer: Option<MeshRenderer>,
        camera: Option<Camera>,
        rotate_speed: Vec3,
    ) -> EntityId {
        let entity = EntityId(self.next_entity_id);
        self.next_entity_id += 1;
        self.entities.push(entity);

        self.transform.insert(entity, transform);
        self.visibility.insert(entity, Visibility::default());

        if let Some(mesh_renderer) = mesh_renderer {
            self.mesh_renderer.insert(entity, mesh_renderer);
        }

        if let Some(camera) = camera {
            self.camera.insert(entity, camera);
        }

        if rotate_speed != vec3(0.0, 0.0, 0.0) {
            self.rotator.insert(
                entity,
                Rotator {
                    speed: rotate_speed,
                },
            );
        }

        entity
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

    pub fn contains(&self, entity: EntityId) -> bool {
        self.entities.contains(&entity)
    }

    pub fn entity_count(&self) -> usize {
        self.entities.len()
    }


    // Component access ////////////////////////////////////////////
    pub fn add_component<T: WorldComponent>(&mut self, entity: EntityId, component: T) -> bool {
        if !self.contains(entity) {
            return false;
        }

        T::pool_mut(self).insert(entity, component);
        true
    }

    pub fn remove_component<T: WorldComponent>(&mut self, entity: EntityId) -> Option<T> {
        T::pool_mut(self).remove(entity)
    }

    pub fn get_component<T: WorldComponent>(&self, entity: EntityId) -> Option<&T> {
        T::pool(self).get(entity)
    }

    pub fn get_component_mut<T: WorldComponent>(&mut self, entity: EntityId) -> Option<&mut T> {
        T::pool_mut(self).get_mut(entity)
    }

    pub fn has_component<T: WorldComponent>(&self, entity: EntityId) -> bool {
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

    pub fn update(&mut self, delta_time: f32) -> Result<()> {
        for (entity, rotator) in self.rotator.iter() {
            if let Some(transform) = self.transform.get_mut(entity) {
                transform.rotate(rotator.speed * delta_time);
            }
        }

        Ok(())
    }
}

impl Default for World {
    fn default() -> Self {
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

impl WorldComponent for Transform {
    fn pool(world: &World) -> &ComponentPool<Self> {
        &world.transform
    }

    fn pool_mut(world: &mut World) -> &mut ComponentPool<Self> {
        &mut world.transform
    }
}

impl WorldComponent for Camera {
    fn pool(world: &World) -> &ComponentPool<Self> {
        &world.camera
    }

    fn pool_mut(world: &mut World) -> &mut ComponentPool<Self> {
        &mut world.camera
    }
}

impl WorldComponent for Material {
    fn pool(world: &World) -> &ComponentPool<Self> {
        &world.material
    }

    fn pool_mut(world: &mut World) -> &mut ComponentPool<Self> {
        &mut world.material
    }
}

impl WorldComponent for MeshRenderer {
    fn pool(world: &World) -> &ComponentPool<Self> {
        &world.mesh_renderer
    }

    fn pool_mut(world: &mut World) -> &mut ComponentPool<Self> {
        &mut world.mesh_renderer
    }
}

impl WorldComponent for Rotator {
    fn pool(world: &World) -> &ComponentPool<Self> {
        &world.rotator
    }

    fn pool_mut(world: &mut World) -> &mut ComponentPool<Self> {
        &mut world.rotator
    }
}

impl WorldComponent for Visibility {
    fn pool(world: &World) -> &ComponentPool<Self> {
        &world.visibility
    }

    fn pool_mut(world: &mut World) -> &mut ComponentPool<Self> {
        &mut world.visibility
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

    fn spawn_renderable(world: &mut World, mesh: MeshHandle, transform: Transform) -> EntityId {
        world.spawn(
            transform,
            Some(MeshRenderer {
                mesh,
                material: Material::default(),
            }),
            None,
            vec3(20.0, 0.0, 0.0),
        )
    }

    #[test]
    fn spawn_adds_entity_and_returns_unique_entity_ids() {
        let mut world = World::default();
        let mesh = mesh_handle(0);

        let first = spawn_renderable(&mut world, mesh, Transform::default());
        let second = spawn_renderable(&mut world, mesh, Transform::default());

        assert_eq!(world.entity_count(), 2);
        assert_ne!(first, second);
        assert_eq!(first, EntityId(0));
        assert_eq!(second, EntityId(1));
    }

    #[test]
    fn shortcuts_return_components_for_entity() {
        let mut world = World::default();
        let mesh = mesh_handle(7);
        let transform = Transform {
            position: vec3(0.0, 1.0, 1.0),
            rotation: vec3(-1.0, 2.0, 3.0),
            scale: vec3(1.0, 2.0, 3.0),
        };

        let entity = spawn_renderable(&mut world, mesh, transform.clone());

        assert_eq!(world.mesh_renderer(entity).unwrap().mesh, mesh);
        assert_eq!(
            world.transform(entity).unwrap().position,
            transform.position
        );
        assert_eq!(
            world.transform(entity).unwrap().rotation,
            transform.rotation
        );
        assert_eq!(world.transform(entity).unwrap().scale, transform.scale);
    }

    #[test]
    fn transform_mut_can_change_transform() {
        let mut world = World::default();
        let entity = spawn_renderable(&mut world, mesh_handle(0), Transform::default());

        world.transform_mut(entity).unwrap().position = vec3(1.0, 2.0, 3.0);

        assert_eq!(
            world.transform(entity).unwrap().position,
            vec3(1.0, 2.0, 3.0)
        );
    }

    #[test]
    fn despawn_removes_entity_and_components() {
        let mut world = World::default();
        let first = spawn_renderable(&mut world, mesh_handle(0), Transform::default());
        let second = spawn_renderable(&mut world, mesh_handle(1), Transform::default());

        assert!(world.despawn(first));

        assert_eq!(world.entity_count(), 1);
        assert!(!world.contains(first));
        assert!(world.contains(second));
        assert!(world.transform(first).is_none());
        assert!(world.mesh_renderer(first).is_none());
    }

    #[test]
    fn despawn_unknown_entity_returns_false() {
        let mut world = World::default();

        assert!(!world.despawn(EntityId(999)));
        assert_eq!(world.entity_count(), 0);
    }

    #[test]
    fn component_access_can_add_get_and_remove_components() {
        let mut world = World::default();
        let entity = world.spawn(Transform::default(), None, None, vec3(0.0, 0.0, 0.0));

        assert!(world.add_component(
            entity,
            Rotator {
                speed: vec3(1.0, 2.0, 3.0),
            }
        ));
        assert!(world.has_component::<Rotator>(entity));
        assert_eq!(world.get_component::<Rotator>(entity).unwrap().speed.x, 1.0);
        assert!(world.remove_component::<Rotator>(entity).is_some());
        assert!(!world.has_component::<Rotator>(entity));
    }

    #[test]
    fn renderables_iterates_mesh_renderer_entities_with_transform() {
        let mut world = World::default();
        let renderable = spawn_renderable(&mut world, mesh_handle(0), Transform::default());
        world.spawn(Transform::default(), None, None, vec3(0.0, 0.0, 0.0));

        let renderables = world.renderables().collect::<Vec<_>>();

        assert_eq!(renderables.len(), 1);
        assert_eq!(renderables[0].entity, renderable);
    }

    #[test]
    fn update_rotates_entities_with_rotator() {
        let mut world = World::default();
        let entity = world.spawn(
            Transform::default(),
            Some(MeshRenderer {
                mesh: mesh_handle(0),
                material: Material::default(),
            }),
            None,
            vec3(40.0, 0.0, 0.0),
        );

        world.update(0.5).unwrap();

        assert_eq!(
            world.transform(entity).unwrap().rotation,
            vec3(20.0, 0.0, 0.0)
        );
    }
}
