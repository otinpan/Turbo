#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct EntityId(pub usize);

impl EntityId {
    pub const fn value(self) -> usize {
        self.0
    }

    pub(crate) const fn from_index(index: usize) -> Self {
        Self(index)
    }

    pub const fn index(self) -> usize {
        self.0
    }
}
