use renderer_vulkan::VertexLayout;

use crate::{PrimitiveShape, PrimitiveType, RenderCommandQueue};

pub trait RenderCommandApi {
    fn render_commands_mut(&mut self) -> &mut RenderCommandQueue;

    fn update_primitive_mesh(
        &mut self,
        primitive_type: PrimitiveType,
        vertex_layout: VertexLayout,
        shape: PrimitiveShape,
    ) {
        self.render_commands_mut()
            .update_primitive_mesh(primitive_type, vertex_layout, shape);
    }
}
