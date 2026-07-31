use super::Material;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct MeshHandle(pub usize);

#[derive(Clone, Debug)]
pub struct MeshRenderer {
    pub mesh: MeshHandle,
    pub material: Material,
}

impl MeshRenderer {
    pub fn new(mesh: MeshHandle, material: Material) -> Self {
        Self { mesh, material }
    }

    pub fn default_material(mesh: MeshHandle) -> Self {
        Self {
            mesh,
            material: Material::default(),
        }
    }
}
