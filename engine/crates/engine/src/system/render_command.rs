use renderer_vulkan::{MeshHandle, VertexLayout};

use crate::{PrimitiveShape, PrimitiveType};

#[derive(Debug)]
pub enum RenderCommand {
    DestroyMesh {
        mesh: MeshHandle,
    },
    UpdatePrimitiveMesh {
        primitive_type: PrimitiveType,
        vertex_layout: VertexLayout,
        shape: PrimitiveShape,
    },
}

#[derive(Debug)]
pub struct RenderCommandQueue {
    command: Vec<RenderCommand>,
}

impl RenderCommandQueue {
    pub fn destroy_mesh(&mut self, mesh: MeshHandle) {
        self.command.push(RenderCommand::DestroyMesh { mesh });
    }

    pub fn update_primitive_mesh(
        &mut self,
        primitive_type: PrimitiveType,
        vertex_layout: VertexLayout,
        shape: PrimitiveShape,
    ) {
        self.command.push(RenderCommand::UpdatePrimitiveMesh {
            primitive_type,
            vertex_layout,
            shape,
        });
    }

    pub fn drain(&mut self) -> impl Iterator<Item = RenderCommand> + '_ {
        self.command.drain(..)
    }
}

impl Default for RenderCommandQueue {
    fn default() -> Self {
        Self {
            command: Vec::new(),
        }
    }
}
