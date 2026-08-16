use super::Component;
use super::Material;
use anyhow::{Result, bail};
use renderer_vulkan::MeshHandle;

use crate::MeshAssetId;

#[derive(Clone, Debug)]
pub struct MeshRenderer {
    pub mesh: MeshHandle,
    pub asset_id: Option<MeshAssetId>,
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

        Ok(Self {
            mesh,
            asset_id: None,
            material,
        })
    }

    pub fn with_asset_id(mut self, asset_id: MeshAssetId) -> Self {
        self.asset_id = Some(asset_id);
        self
    }
}

impl Component for MeshRenderer {}
