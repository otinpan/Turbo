use super::EntityId;
const INVALID: usize = usize::MAX;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ComponentPool<T> {
    entities: Vec<EntityId>, // [1,4,7,...]
    components: Vec<T>,      // [T,T,T,...]
    sparse: Vec<usize>,      // [INV,0,INV,INV,1,...]
}

impl<T> ComponentPool<T> {
    pub fn new() -> Self {
        Self {
            entities: Vec::new(),
            components: Vec::new(),
            sparse: Vec::new(),
        }
    }

    pub fn insert(&mut self, entity: EntityId, component: T) {
        let id = entity.0;
        if id >= self.sparse.len() {
            self.sparse.resize(id + 1, INVALID)
        }

        let index = self.sparse[id];

        // already exist
        if index != INVALID {
            self.components[index] = component;
            return;
        }

        let index = self.components.len();
        self.sparse[id] = index;
        self.components.push(component);
        self.entities.push(entity);
    }

    pub fn remove(&mut self, entity: EntityId) -> Option<T> {
        let id = entity.0;
        if id >= self.sparse.len() {
            return None;
        }

        let index = self.sparse[id];
        if index == INVALID {
            return None;
        }

        self.sparse[id] = INVALID;

        let component = self.components.swap_remove(index);
        self.entities.swap_remove(index);

        if index < self.entities.len() {
            let moved_entity = self.entities[index];
            self.sparse[moved_entity.0] = index;
        }

        Some(component)
    }

    pub fn get(&self, entity: EntityId) -> Option<&T> {
        let id = entity.0;
        if id >= self.sparse.len() {
            return None;
        }

        let index = self.sparse[id];
        if index == INVALID {
            return None;
        }

        self.components.get(index)
    }

    pub fn get_mut(&mut self, entity: EntityId) -> Option<&mut T> {
        let id = entity.0;
        if id >= self.sparse.len() {
            return None;
        }

        let index = self.sparse[id];
        if index == INVALID {
            return None;
        }
        self.components.get_mut(index)
    }

    pub fn contains(&self, entity: EntityId) -> bool {
        let id = entity.0;
        if id >= self.sparse.len() {
            return false;
        }
        self.sparse[id] != INVALID
    }

    pub fn iter(&self) -> impl Iterator<Item = (EntityId, &T)> {
        self.entities.iter().copied().zip(self.components.iter())
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = (EntityId, &mut T)> {
        self.entities
            .iter()
            .copied()
            .zip(self.components.iter_mut())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn remove_returns_component_and_clears_entity_slot() {
        let mut pool = ComponentPool::new();
        let entity = EntityId(2);

        pool.insert(entity, "transform");

        assert_eq!(pool.remove(entity), Some("transform"));
        assert_eq!(pool.remove(entity), None);
    }

    #[test]
    fn remove_updates_sparse_index_for_swapped_entity() {
        let mut pool = ComponentPool::new();
        let first = EntityId(1);
        let second = EntityId(4);

        pool.insert(first, 10);
        pool.insert(second, 20);

        assert_eq!(pool.remove(first), Some(10));
        assert_eq!(pool.remove(second), Some(20));
    }

    #[test]
    fn check_whole_functions() {
        let mut pool = ComponentPool::new();
        let e0 = EntityId(0);
        let e1 = EntityId(4);
        let e2 = EntityId(7);
        let e3 = EntityId(5);
        let e4 = EntityId(2);
        let ef = EntityId(1);

        pool.insert(e0, 10);
        pool.insert(e1, 8);
        pool.insert(e2, 256);
        pool.insert(e3, 35);
        pool.insert(e4, 1);

        // check insert
        assert_eq!(pool.entities, vec![e0, e1, e2, e3, e4]);
        assert_eq!(pool.components, vec![10, 8, 256, 35, 1]);
        assert_eq!(pool.sparse, vec![0, INVALID, 4, INVALID, 1, 3, INVALID, 2]);

        // check contain
        assert!(pool.contains(e2));
        assert!(!pool.contains(ef));

        // check get
        assert_eq!(pool.get(e3), Some(&35));
        assert_eq!(pool.get(ef), None);

        // check iter
        let items: Vec<_> = pool.iter().map(|(e, c)| (e, *c)).collect();
        assert_eq!(items, vec![(e0, 10), (e1, 8), (e2, 256), (e3, 35), (e4, 1)]);

        pool.remove(e2);
        // swap_remove
        assert_eq!(pool.entities, vec![e0, e1, e4, e3]);
        assert_eq!(pool.components, vec![10, 8, 1, 35]);
        assert_eq!(
            pool.sparse,
            vec![0, INVALID, 2, INVALID, 1, 3, INVALID, INVALID]
        );
    }

    #[test]
    fn remove_unknown_entity_returns_none() {
        let mut pool = ComponentPool::<u32>::new();

        assert_eq!(pool.remove(EntityId(99)), None);
    }
}
