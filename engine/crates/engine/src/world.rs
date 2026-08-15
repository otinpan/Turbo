use super::{EntityId, Registry};

pub type Vec3 = cgmath::Vector3<f32>;

// World ///////////////////////////////////////////
#[derive(Debug)]
pub struct World {
    pub registry: Registry,
}

impl World {
    pub fn spawn(&mut self) -> EntityId {
        self.registry.create()
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
