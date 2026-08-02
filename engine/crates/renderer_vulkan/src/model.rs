use anyhow::Result;
use cgmath::{vec2, vec3};
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;

use super::vertex::{DebugLineVertex, Mesh3DVertex, SourceVertex};

#[derive(Clone, Debug)]
pub struct SourceMesh {
    pub vertices: Vec<SourceVertex>,
    pub indices: Vec<u32>,
}

impl SourceMesh {
    pub fn to_mesh3d_data(&self) -> MeshData<Mesh3DVertex> {
        MeshData {
            vertices: self.vertices.iter().map(Mesh3DVertex::from).collect(),
            indices: self.indices.clone(),
        }
    }

    pub fn to_debugline_data(&self) -> MeshData<DebugLineVertex> {
        MeshData {
            vertices: self.vertices.iter().map(DebugLineVertex::from).collect(),
            indices: triangle_indices_to_line_indices(&self.indices),
        }
    }
}

fn triangle_indices_to_line_indices(indices: &[u32]) -> Vec<u32> {
    let mut line_indices = Vec::with_capacity(indices.len() * 2);

    for triangle in indices.chunks_exact(3) {
        let a = triangle[0];
        let b = triangle[1];
        let c = triangle[2];

        line_indices.extend_from_slice(&[a, b, b, c, c, a]);
    }

    line_indices
}

#[derive(Clone, Debug)]
pub struct MeshData<V> {
    pub vertices: Vec<V>,
    pub indices: Vec<u32>,
}

pub fn load_model_source(file_path: &str) -> Result<SourceMesh> {
    let mut reader = BufReader::new(File::open(file_path)?);

    let (models, _) = tobj::load_obj_buf(
        &mut reader,
        &tobj::LoadOptions {
            triangulate: true,
            ..Default::default()
        },
        |_| Ok(Default::default()),
    )?;

    let mut unique_vertices = HashMap::new();
    let mut vertices: Vec<SourceVertex> = vec![];
    let mut indices: Vec<u32> = vec![];

    for model in &models {
        for index in &model.mesh.indices {
            let pos_offset = (3 * index) as usize;
            let tex_coord_offset = (2 * index) as usize;

            let vertex = SourceVertex {
                pos: vec3(
                    model.mesh.positions[pos_offset],
                    model.mesh.positions[pos_offset + 1],
                    model.mesh.positions[pos_offset + 2],
                ),
                color: vec3(1.0, 1.0, 1.0),
                tex_coord: vec2(
                    model.mesh.texcoords[tex_coord_offset],
                    1.0 - model.mesh.texcoords[tex_coord_offset + 1],
                ),
            };

            if let Some(index) = unique_vertices.get(&vertex) {
                indices.push(*index as u32);
            } else {
                let index = vertices.len();
                unique_vertices.insert(vertex, index);
                vertices.push(vertex);
                indices.push(index as u32);
            }
        }
    }
    Ok(SourceMesh { vertices, indices })
}

#[cfg(test)]
mod tests {
    use super::*;
    use cgmath::{vec2, vec3};

    #[test]
    fn to_debugline_data_converts_triangle_indices_to_line_indices() {
        let source = SourceMesh {
            vertices: vec![
                SourceVertex::new(vec3(0.0, 0.0, 0.0), vec3(1.0, 1.0, 1.0), vec2(0.0, 0.0)),
                SourceVertex::new(vec3(1.0, 0.0, 0.0), vec3(1.0, 1.0, 1.0), vec2(1.0, 0.0)),
                SourceVertex::new(vec3(0.0, 1.0, 0.0), vec3(1.0, 1.0, 1.0), vec2(0.0, 1.0)),
            ],
            indices: vec![0, 1, 2],
        };

        let mesh_data = source.to_debugline_data();

        assert_eq!(mesh_data.vertices.len(), 3);
        assert_eq!(mesh_data.indices, vec![0, 1, 1, 2, 2, 0]);
    }
}
