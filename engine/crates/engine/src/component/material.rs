use cgmath::Vector3;
use renderer_vulkan::{PipelineKey, TextureHandle};

use super::Component;
use crate::app::DEFAULT_TEXTURE;
pub type Vec3 = Vector3<f32>;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Material {
    pub color: Vec3,
    pub alpha: f32,
    pub use_texture: bool,
    pub texture: TextureHandle,
    pub pipeline_key: PipelineKey,
}

impl Material {
    pub const fn new(
        color: Vec3,
        alpha: f32,
        use_texture: bool,
        texture: TextureHandle,
        pipeline_key: PipelineKey,
    ) -> Self {
        Self {
            color,
            alpha,
            use_texture,
            texture,
            pipeline_key,
        }
    }
}

impl Component for Material {}

impl Default for Material {
    fn default() -> Self {
        Self {
            color: cgmath::vec3(1.0, 1.0, 1.0),
            alpha: 1.0,
            use_texture: false,
            texture: DEFAULT_TEXTURE,
            pipeline_key: PipelineKey::Mesh3D,
        }
    }
}
