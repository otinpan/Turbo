use super::Material;
use anyhow::{Result, bail};
use renderer_vulkan::{MeshHandle, VertexLayout};

pub fn pipeline_key_for_layout(vertex_layout: VertexLayout) -> renderer_vulkan::PipelineKey {
    match vertex_layout {
        VertexLayout::Mesh3D => renderer_vulkan::PipelineKey::Mesh3D,
        VertexLayout::DebugLine3D => renderer_vulkan::PipelineKey::DebugLine3D,
    }
}

#[derive(Clone, Debug)]
pub struct MeshRenderer {
    pub mesh: MeshHandle,
    pub material: Material,
}

impl MeshRenderer {
    pub fn new(mesh: MeshHandle, material: Material) -> Result<Self> {
        let expected_pipeline = pipeline_key_for_layout(mesh.vertex_layout);

        if material.pipeline_key != expected_pipeline {
            bail!(
                "Material pipeline {:?} does not match mesh vertex layout {:?}.",
                material.pipeline_key,
                mesh.vertex_layout
            );
        }

        Ok(Self { mesh, material })
    }

    pub fn default_material(mesh: MeshHandle) -> Self {
        Self {
            mesh,
            material: Material {
                pipeline_key: pipeline_key_for_layout(mesh.vertex_layout),
                ..Material::default()
            },
        }
    }
}
