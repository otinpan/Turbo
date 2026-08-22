// Meshes are created in RenderSystem.
// PendingPrimitiveMesh offer RenderSystem to create new primitive mesh and attach entity new MeshRenderer component
use super::Component;
use crate::{Material, PrimitiveShape};

#[derive(Clone, Debug)]
pub struct PendingPrimitiveMesh {
    pub shape: PrimitiveShape,
    pub material: Material,
    pub auto_release: bool,
}

impl Component for PendingPrimitiveMesh {}
