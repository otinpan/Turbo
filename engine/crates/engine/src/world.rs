use super::{Component, EntityId, Name, Registry, Tags};

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

    pub fn find_by_name(&self, target: &str) -> Option<EntityId> {
        self.registry
            .get_pool::<Name>()?
            .iter()
            .find(|(_, name)| name.value == target)
            .map(|(entity, _)| entity)
    }

    pub fn set_name(&mut self, entity: EntityId, name: &str) -> bool {
        if self.find_by_name(name).is_some() {
            return false;
        }

        self.add_component(entity, Name::new(name))
    }

    pub fn remove_name(&mut self, entity: EntityId) -> bool {
        self.remove_component::<Name>(entity).is_some()
    }

    pub fn set_tags<const N: usize>(&mut self, entity: EntityId, tags: [&str; N]) -> bool {
        self.add_component(entity, Tags::new(tags))
    }

    pub fn remove_tags(&mut self, entity: EntityId) -> bool {
        self.remove_component::<Tags>(entity).is_some()
    }

    pub fn remove_tag(&mut self, entity: EntityId, tag: &str) -> bool {
        let Some(tags) = self.get_component_mut::<Tags>(entity) else {
            return false;
        };

        tags.remove(tag)
    }

    pub fn find_by_tag(&self, target: &str) -> Vec<EntityId> {
        self.registry
            .get_pool::<Tags>()
            .map(|pool| {
                pool.iter()
                    .filter(|(_, tags)| tags.contains(target))
                    .map(|(entity, _)| entity)
                    .collect()
            })
            .unwrap_or_default()
    }

    pub fn get_all_named_entities(&self) -> Vec<(String, EntityId)> {
        let mut entries = Vec::new();
        if let Some(pool) = self.registry.get_pool::<Name>() {
            for (entity, entity_name) in pool.iter() {
                entries.push((entity_name.value.clone(), entity))
            }
        }

        entries
    }

    pub fn get_all_taged_entities(&self) -> Vec<(String, EntityId)> {
        let mut entries = Vec::new();

        if let Some(pool) = self.registry.get_pool::<Tags>() {
            for (entity, entity_tags) in pool.iter() {
                for tag in &entity_tags.values {
                    entries.push((tag.clone(), entity));
                }
            }
        }

        entries
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

    #[test]
    fn get_component_name_and_tags() {
        let mut world = World::default();
        let camera = world.spawn();
        assert!(world.set_name(camera, "Camera"));

        let obj0 = world.spawn();
        assert!(world.set_tags(obj0, ["Object", "Primitive"]));
        assert!(world.set_name(obj0, "OriginalObject"));
        let obj1 = world.spawn();
        assert!(world.set_tags(obj1, ["Object", "Primitive"]));
        let obj2 = world.spawn();
        assert!(world.set_tags(obj2, ["Object", "Model"]));

        assert_eq!(world.find_by_name("Camera"), Some(camera));
        assert_eq!(world.find_by_tag("Primitive"), vec![obj0, obj1]);
    }

    #[test]
    fn remove_name_and_tags() {
        let mut world = World::default();
        let entity = world.spawn();

        assert!(world.set_name(entity, "Object"));
        assert!(world.set_tags(entity, ["Object", "Primitive"]));

        assert!(world.remove_name(entity));
        assert!(!world.remove_name(entity));
        assert_eq!(world.find_by_name("Object"), None);

        assert!(world.remove_tag(entity, "Primitive"));
        assert!(!world.remove_tag(entity, "Primitive"));
        assert_eq!(world.find_by_tag("Primitive"), Vec::<EntityId>::new());

        assert!(world.remove_tags(entity));
        assert!(!world.remove_tags(entity));
        assert_eq!(world.find_by_tag("Object"), Vec::<EntityId>::new());
    }
}
