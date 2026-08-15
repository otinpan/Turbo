use super::{Component, EntityId, Registry};

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

    pub fn add_component<T: Component>(&mut self, entity: EntityId, component: T) -> bool {
        self.registry.add_component(entity, component)
    }

    pub fn remove_component<T: Component>(&mut self, entity: EntityId) -> Option<T> {
        self.registry.remove_component::<T>(entity)
    }

    pub fn get_component<T: Component>(&self, entity: EntityId) -> Option<&T> {
        self.registry.get_component::<T>(entity)
    }

    pub fn get_component_mut<T: Component>(&mut self, entity: EntityId) -> Option<&mut T> {
        self.registry.get_component_mut::<T>(entity)
    }

    pub fn has_component<T: Component>(&self, entity: EntityId) -> bool {
        self.registry.has_component::<T>(entity)
    }
}

impl Default for World {
    fn default() -> Self {
        Self {
            registry: Registry::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, PartialEq)]
    struct Health {
        hp: i32,
    }

    impl Component for Health {}

    #[test]
    fn component_access_delegates_to_registry() {
        let mut world = World::default();
        let entity = world.spawn();

        assert!(world.add_component(entity, Health { hp: 100 }));
        assert!(world.has_component::<Health>(entity));
        assert_eq!(
            world.get_component::<Health>(entity),
            Some(&Health { hp: 100 })
        );

        world.get_component_mut::<Health>(entity).unwrap().hp -= 25;
        assert_eq!(world.get_component::<Health>(entity).unwrap().hp, 75);

        assert_eq!(
            world.remove_component::<Health>(entity),
            Some(Health { hp: 75 })
        );
        assert!(!world.has_component::<Health>(entity));
    }

}
