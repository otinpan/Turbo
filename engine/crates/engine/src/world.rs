use anyhow::Result;
use turbo_math::Transform;
pub type Vec3 = cgmath::Vector3<f32>;

use super::MeshRenderer;
use super::CameraComponent;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct EntityId(u64);

// World Object ////////////////////////////////////
#[derive(Clone, Debug)]
pub struct WorldObject {
    pub id: EntityId,
    pub transform: Transform,
    pub mesh_renderer: Option<MeshRenderer>,
    pub camera: Option<CameraComponent>,
    rotate_speed: Vec3,
}




// World ///////////////////////////////////////////
#[derive(Clone, Debug)]
pub struct World {
    next_entity_id: u64,
    objects: Vec<WorldObject>,
}


impl World {
    pub fn spawn(
        &mut self,
        transform: Transform,
        mesh_renderer: Option<MeshRenderer>,
        camera: Option<CameraComponent>,
        rotate_speed: Vec3,
    ) -> EntityId {
        let id = EntityId(self.next_entity_id);
        self.next_entity_id += 1;

        self.objects.push(WorldObject {
            id,
            transform,
            mesh_renderer,
            camera,
            rotate_speed,
        });

        id
    }

    pub fn despawn(&mut self, id: EntityId) -> Option<WorldObject> {
        let index = self.objects.iter().position(|object| object.id == id)?;
        // swap index and last, then pop back
        Some(self.objects.swap_remove(index))
    }

    pub fn objects(&self) -> &[WorldObject] {
        &self.objects
    }

    pub fn object_count(&self) -> usize {
        self.objects.len()
    }

    pub fn get(&self, id: EntityId) -> Option<&WorldObject> {
        self.objects.iter().find(|object| object.id == id)
    }

    pub fn get_mut(&mut self, id: EntityId) -> Option<&mut WorldObject> {
        self.objects.iter_mut().find(|object| object.id == id)
    }

    pub fn active_camera(&self) -> Option<&WorldObject>{
        self.objects.iter().find(|object| object.camera.is_some())
    }

    pub fn update(&mut self, delta_time: f32) -> Result<()> {
        for object in &mut self.objects {
            object.transform.rotate(object.rotate_speed * delta_time);
        }

        Ok(())
    }
}

impl Default for World {
    fn default() -> Self {
        Self {
            next_entity_id: 0,
            objects: Vec::new(),
        }
    }
}


// test ///////////////////////////////////////////
#[cfg(test)]
mod tests {
    use super::*;
    use crate::MeshHandle;
    use cgmath::vec3;

    fn spawn_renderable(world: &mut World, mesh: MeshHandle, transform: Transform) -> EntityId {
        world.spawn(
            transform,
            Some(MeshRenderer { mesh }),
            None,
            vec3(20.0, 0.0, 0.0),
        )
    }

    #[test]
    fn spawn_adds_object_and_returns_unique_entity_ids() {
        let mut world = World::default();
        let mesh = MeshHandle(0);

        let first = spawn_renderable(&mut world, mesh, Transform::default());
        let second = spawn_renderable(&mut world, mesh, Transform::default());

        assert_eq!(world.object_count(), 2);
        assert_ne!(first, second);
        assert_eq!(first, EntityId(0));
        assert_eq!(second, EntityId(1));
    }

    #[test]
    fn get_returns_object_for_entity_id() {
        let mut world = World::default();
        let mesh = MeshHandle(7);
        let transform = Transform {
            position: vec3(0.0, 1.0, 1.0),
            rotation: vec3(-1.0, 2.0, 3.0),
            scale: vec3(1.0, 2.0, 3.0),
        };

        let id = spawn_renderable(&mut world, mesh, transform.clone());

        let object = world.get(id).expect("spawned entity should exist");
        assert_eq!(object.id, id);
        assert_eq!(
            object
                .mesh_renderer
                .as_ref()
                .expect("spawned entity should have a mesh renderer")
                .mesh,
            mesh
        );
        assert_eq!(object.transform.position, transform.position);
        assert_eq!(object.transform.rotation, transform.rotation);
        assert_eq!(object.transform.scale, transform.scale);
    }

    #[test]
    fn get_mut_can_change_object_transform() {
        let mut world = World::default();
        let id = spawn_renderable(&mut world, MeshHandle(0), Transform::default());

        world
            .get_mut(id)
            .expect("spawned entity should exist")
            .transform
            .position = vec3(1.0, 2.0, 3.0);

        assert_eq!(
            world.get(id).expect("spawned entity should exist").transform.position,
            vec3(1.0, 2.0, 3.0)
        );
    }

    #[test]
    fn despawn_removes_only_matching_entity() {
        let mut world = World::default();
        let first = spawn_renderable(&mut world, MeshHandle(0), Transform::default());
        let second = spawn_renderable(&mut world, MeshHandle(1), Transform::default());

        let removed = world.despawn(first).expect("entity should be removed");

        assert_eq!(removed.id, first);
        assert_eq!(world.object_count(), 1);
        assert!(world.get(first).is_none());
        assert!(world.get(second).is_some());
    }

    #[test]
    fn despawn_unknown_entity_returns_none() {
        let mut world = World::default();

        assert!(world.despawn(EntityId(999)).is_none());
        assert_eq!(world.object_count(), 0);
    }

    #[test]
    fn update_rotates_objects_by_their_rotate_speed() {
        let mut world = World::default();
        let id = world.spawn(
            Transform::default(),
            Some(MeshRenderer {
                mesh: MeshHandle(0),
            }),
            None,
            vec3(40.0, 0.0, 0.0),
        );

        world.update(0.5).unwrap();

        assert_eq!(
            world.get(id).expect("spawned entity should exist").transform.rotation,
            vec3(20.0, 0.0, 0.0)
        );
    }
}
