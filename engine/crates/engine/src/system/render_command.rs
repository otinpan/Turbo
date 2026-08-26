use renderer_vulkan::MeshHandle;

use crate::{EntityId, MeshAssetId, PrimitiveShape};

#[derive(Debug)]
pub enum RenderCommand {
    DestroyMesh {
        mesh: MeshHandle,
    },
    UpdatePrimitiveMesh {
        asset_id: MeshAssetId,
        shape: PrimitiveShape,
    },
    CreatePrimitiveMesh {
        entity: EntityId,
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

    pub fn update_primitive_mesh(&mut self, asset_id: MeshAssetId, shape: PrimitiveShape) {
        self.command
            .push(RenderCommand::UpdatePrimitiveMesh { asset_id, shape });
    }

    pub fn create_primitive_mesh(&mut self, entity: EntityId) {
        self.command
            .push(RenderCommand::CreatePrimitiveMesh { entity });
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
