use crate::{PrimitiveShape, MeshAssetId, RenderCommandQueue};

pub trait RenderCommandApi {
    fn render_commands_mut(&mut self) -> &mut RenderCommandQueue;

    fn update_primitive_mesh(
        &mut self,
        asset_id: MeshAssetId,
        shape: PrimitiveShape,
    ) {
        self.render_commands_mut()
            .update_primitive_mesh(asset_id, shape);
    }
}
