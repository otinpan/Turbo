use anyhow::Result;
use vulkanalia::prelude::v1_0::*;

use super::index::create_index_buffer;
use super::model::MeshData;
use super::types::{Mesh, VulkanData};
use super::vertex::create_vertex_buffer;

pub unsafe fn create_mesh(
    instance: &Instance,
    device: &Device,
    data: &VulkanData,
    mesh_data: MeshData,
) -> Result<Mesh> {
    let index_count = mesh_data.indices.len() as u32;

    let (vertex_buffer, vertex_buffer_memory) =
        create_vertex_buffer(instance, device, data, &mesh_data.vertices)?;

    let (index_buffer, index_buffer_memory) =
        create_index_buffer(instance, device, data, &mesh_data.indices)?;

    Ok(Mesh {
        vertices: mesh_data.vertices,
        indices: mesh_data.indices,
        vertex_buffer,
        vertex_buffer_memory,
        index_buffer,
        index_buffer_memory,
        index_count,
    })
}
