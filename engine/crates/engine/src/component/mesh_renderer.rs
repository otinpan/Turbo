use super::Component;
use super::Material;
use anyhow::{Result, bail};
use renderer_vulkan::MeshHandle;

#[derive(Clone, Debug)]
pub struct MeshRenderer {
    pub mesh: MeshHandle,
    pub material: Material,
}

impl MeshRenderer {
    pub fn new(mesh: MeshHandle, material: Material) -> Result<Self> {
        if material.pipeline_key.required_vertex_layout() != mesh.vertex_layout {
            bail!(
                "Material pipeline {:?} does not match mesh vertex layout {:?}.",
                material.pipeline_key,
                mesh.vertex_layout
            );
        }

        Ok(Self { mesh, material })
    }
}

impl Component for MeshRenderer {}
