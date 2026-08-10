#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct EntityId(pub(crate) u64);

impl EntityId {
    pub const fn value(self) -> u64 {
        self.0
    }

    pub(crate) const fn from_index(index: u64) -> Self {
        Self(index)
    }

    pub const fn index(self) -> u64 {
        self.0
    }
}