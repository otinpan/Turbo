use crate::{PrimitiveMesh, PrimitiveType};
use anyhow::Result;
use cgmath::Vector3;
use renderer_vulkan::{
    MeshHandle, SkyboxTextureHandle, TextureHandle, VertexLayout, VulkanRenderer,
};
use std::collections::HashMap;

pub type Vec3 = Vector3<f32>;

// handle the number of entities that use this Mesh
#[derive(Debug)]
pub struct MeshAsset {
    pub handle: MeshHandle,
    pub ref_count: usize,
    pub auto_release: bool,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct MeshAssetId(pub usize);

pub struct Resources {
    pub mesh_assets: Vec<Option<MeshAsset>>,
    pub models: HashMap<String, MeshAssetId>,
    pub textures: HashMap<String, TextureHandle>,
    pub primitive_meshes: Vec<PrimitiveMesh>,
    pub skybox_mesh: Option<MeshHandle>,
    pub skybox_textures: HashMap<String, SkyboxTextureHandle>,
}

impl Resources {
    pub fn insert_mesh_asset(&mut self, handle: MeshHandle, auto_release: bool) -> MeshAssetId {
        let id = MeshAssetId(self.mesh_assets.len());

        self.mesh_assets.push(Some(MeshAsset {
            handle,
            ref_count: 0,
            auto_release,
        }));

        id
    }

    pub fn register_model(
        &mut self,
        name: impl Into<String>,
        handle: MeshHandle,
        auto_release: bool,
    ) -> MeshAssetId {
        let id = self.insert_mesh_asset(handle, auto_release);
        self.models.insert(name.into(), id);
        id
    }

    pub fn model_asset_id(&self, name: &str) -> Option<MeshAssetId> {
        self.models.get(name).copied()
    }

    pub fn primitive_asset_id(
        &self,
        primitive_type: PrimitiveType,
        vertex_layout: VertexLayout,
    ) -> Option<MeshAssetId> {
        self.primitive_meshes
            .iter()
            .find(|mesh| {
                mesh.primitive_type == primitive_type && mesh.vertex_layout == vertex_layout
            })
            .map(|mesh| mesh.asset_id)
    }

    pub fn get_mesh_handle(&self, id: MeshAssetId) -> Option<MeshHandle> {
        self.mesh_assets
            .get(id.0)?
            .as_ref()
            .map(|asset| asset.handle)
    }

    pub fn retain_mesh(&mut self, id: MeshAssetId) -> Option<MeshHandle> {
        let asset = self.mesh_assets.get_mut(id.0)?.as_mut()?;
        asset.ref_count += 1;
        Some(asset.handle)
    }

    pub fn release_mesh(&mut self, id: MeshAssetId) -> Option<MeshHandle> {
        let asset = self.mesh_assets.get_mut(id.0)?.as_mut()?;

        if asset.ref_count > 0 {
            asset.ref_count -= 1;
        }

        if asset.ref_count == 0 && asset.auto_release {
            let handle = asset.handle;
            self.mesh_assets[id.0] = None;
            return Some(handle);
        }

        None
    }

    pub unsafe fn release_mesh_for_renderer(
        &mut self,
        id: MeshAssetId,
        renderer: &mut VulkanRenderer,
    ) -> Result<()> {
        if let Some(handle) = self.release_mesh(id) {
            log::debug!("Mesh asset released and ready to destroy: {handle:?}");
            renderer.destroy_mesh(handle)?;
        }

        Ok(())
    }
}

impl Default for Resources {
    fn default() -> Self {
        Self {
            mesh_assets: Vec::new(),
            models: HashMap::new(),
            textures: HashMap::new(),
            primitive_meshes: Vec::new(),
            skybox_mesh: None,
            skybox_textures: HashMap::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn mesh_handle(index: usize) -> MeshHandle {
        MeshHandle::new(index, VertexLayout::Mesh3D)
    }

    fn mesh_asset(resources: &Resources, id: MeshAssetId) -> &MeshAsset {
        resources.mesh_assets[id.0].as_ref().unwrap()
    }

    #[test]
    fn insert_mesh_asset_starts_ref_count_at_zero() {
        let mut resources = Resources::default();

        let id = resources.insert_mesh_asset(mesh_handle(0), false);

        assert_eq!(mesh_asset(&resources, id).ref_count, 0);
        assert_eq!(mesh_asset(&resources, id).handle, mesh_handle(0));
    }

    #[test]
    fn retain_mesh_increments_ref_count_and_returns_handle() {
        let mut resources = Resources::default();
        let id = resources.insert_mesh_asset(mesh_handle(0), false);

        assert_eq!(resources.retain_mesh(id), Some(mesh_handle(0)));
        assert_eq!(mesh_asset(&resources, id).ref_count, 1);

        assert_eq!(resources.retain_mesh(id), Some(mesh_handle(0)));
        assert_eq!(mesh_asset(&resources, id).ref_count, 2);
    }

    #[test]
    fn release_mesh_decrements_ref_count_without_auto_release() {
        let mut resources = Resources::default();
        let id = resources.insert_mesh_asset(mesh_handle(0), false);
        resources.retain_mesh(id);
        resources.retain_mesh(id);

        assert_eq!(resources.release_mesh(id), None);
        assert_eq!(mesh_asset(&resources, id).ref_count, 1);

        assert_eq!(resources.release_mesh(id), None);
        assert_eq!(mesh_asset(&resources, id).ref_count, 0);
        assert!(resources.mesh_assets[id.0].is_some());
    }

    #[test]
    fn release_mesh_removes_auto_release_asset_when_ref_count_reaches_zero() {
        let mut resources = Resources::default();
        let id = resources.insert_mesh_asset(mesh_handle(0), true);
        resources.retain_mesh(id);

        assert_eq!(resources.release_mesh(id), Some(mesh_handle(0)));
        assert!(resources.mesh_assets[id.0].is_none());
    }

    #[test]
    fn release_mesh_for_missing_asset_returns_none() {
        let mut resources = Resources::default();

        assert_eq!(resources.release_mesh(MeshAssetId(99)), None);
        assert_eq!(resources.retain_mesh(MeshAssetId(99)), None);
    }
}
