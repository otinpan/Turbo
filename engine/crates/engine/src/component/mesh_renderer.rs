// MeshRenderer ///////////////////////////////////////
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct MeshHandle(pub usize);

#[derive(Clone, Debug)]
pub struct MeshRenderer {
    pub mesh: MeshHandle,
}
