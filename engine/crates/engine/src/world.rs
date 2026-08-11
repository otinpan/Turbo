use cgmath::vec3;
use turbo_math::Transform;

use super::{
    Camera, EntityId, 
    MeshRenderer, Rotator, Visibility, Registry,
};

pub type Vec3=cgmath::Vector3<f32>;


// World ///////////////////////////////////////////
#[derive(Clone, Debug)]
pub struct World {
    pub registry: Registry,
}

impl World {
    pub fn spawn(
        &mut self,
        transform: Transform,
        mesh_renderer: Option<MeshRenderer>,
        camera: Option<Camera>,
        rotate_speed: Vec3,
    ) -> EntityId {
        let entity=self.registry.create();

        self.registry.add_component(entity,transform);
        self.registry.add_component(entity,Visibility::default());


        if let Some(mesh_renderer) = mesh_renderer {
            self.registry.add_component(entity, mesh_renderer);
        }

        if let Some(camera) = camera {
            self.registry.add_component(entity,camera);
        }

        if rotate_speed != vec3(0.0, 0.0, 0.0) {
            self.registry.add_component(
                entity,
                Rotator {
                    speed: rotate_speed,
                },
            );
        }

        entity
    }

    pub fn despawn(&mut self, entity: EntityId) -> bool {
        self.registry.despawn(entity)
    }

}

impl Default for World {
    fn default() -> Self {
        Self {
            registry: Registry::default(),
        }
    }
}

