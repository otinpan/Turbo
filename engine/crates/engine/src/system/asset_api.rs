use anyhow::{Result,anyhow};
use crate::{
    MeshAssetId, Resources, PrimitiveType, MeshAsset,
};
use renderer_vulkan::{VertexLayout, TextureHandle, SkyboxTextureHandle};
use crate::app::{DEFAULT_TEXTURE,DEFAULT_SKYBOX_TEXTURE};

pub trait AssetApi{
    fn resources(&self) -> &Resources;
    fn resources_mut(&mut self) -> &mut Resources;

    fn model_asset_id(&self, model_name: &str) -> Result<MeshAssetId>{
        let asset_id=self
            .resources()
            .model_asset_id(model_name)
            .ok_or_else(||anyhow!("model not found: {model_name}"))?;

        Ok(asset_id)
    }

    // return a MeshAssetId that match (primitive_type, vertex_layout)
    fn primitive_asset_id(
        &self,
        primitive_type: PrimitiveType,
        vertex_layout: VertexLayout,
    ) -> Option<MeshAssetId>{
        self.resources().primitive_asset_id(primitive_type,vertex_layout)
    }

    fn texture(&self, texture_name: &str) -> Result<TextureHandle>{
        self.resources()
            .get_texture_handle(texture_name)
            .ok_or_else(||anyhow!("texture not found: {texture_name}"))
    }

    fn default_texture(&self) -> TextureHandle{
        DEFAULT_TEXTURE
    }

    fn default_skybox_texture(&self) -> SkyboxTextureHandle{
        DEFAULT_SKYBOX_TEXTURE
    }

    fn primitive_type_from_asset_id(&self, asset_id: MeshAssetId) -> Option<PrimitiveType>{
        self.resources().primitive_type_from_asset_id(asset_id)
    }

    fn vertex_layout_from_asset_id(&self, asset_id: MeshAssetId) -> Option<VertexLayout>{
        self.resources().vertex_layout_from_asset_id(asset_id)
    }

    fn mesh_assets(&self) -> impl Iterator<Item = (MeshAssetId, &MeshAsset)>{
        self.resources().mesh_assets()
    }
}