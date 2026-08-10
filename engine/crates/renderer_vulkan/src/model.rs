use anyhow::Result;
use cgmath::{vec2, vec3};
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;

use super::vertex::{
    DebugLineVertex, Lit3DVertex, Mesh3DVertex, SkyboxVertex, SourceVertex, Ui2DVertex,
};

#[derive(Clone, Debug)]
pub struct SourceMesh {
    pub vertices: Vec<SourceVertex>,
    pub indices: Vec<u32>,
    pub topology: SourceTopology, // to distingish primitive line when to_debugline_data
}

#[derive(Clone, Debug)]
pub enum SourceTopology {
    TriangleList,
    LineList,
}

impl SourceMesh {
    pub fn to_mesh3d_data(&self) -> MeshData<Mesh3DVertex> {
        MeshData {
            vertices: self.vertices.iter().map(Mesh3DVertex::from).collect(),
            indices: self.indices.clone(),
        }
    }

    pub fn to_debugline_data(&self) -> MeshData<DebugLineVertex> {
        let indices = match self.topology {
            SourceTopology::TriangleList => triangle_indices_to_line_indices(&self.indices),
            SourceTopology::LineList => self.indices.clone(),
        };

        MeshData {
            vertices: self.vertices.iter().map(DebugLineVertex::from).collect(),
            indices,
        }
    }

    pub fn to_lit3d_data(&self) -> MeshData<Lit3DVertex> {
        MeshData {
            vertices: self.vertices.iter().map(Lit3DVertex::from).collect(),
            indices: self.indices.clone(),
        }
    }

    pub fn to_ui2d_data(&self) -> MeshData<Ui2DVertex> {
        MeshData {
            vertices: self.vertices.iter().map(Ui2DVertex::from).collect(),
            indices: self.indices.clone(),
        }
    }

    pub fn to_skybox_data(&self) -> MeshData<SkyboxVertex> {
        MeshData {
            vertices: self.vertices.iter().map(SkyboxVertex::from).collect(),
            indices: self.indices.clone(),
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
            single_index: true,
            ..Default::default()
        },
        |_| Ok(Default::default()),
    )?;

    let mut unique_vertices = HashMap::new();
    let mut vertices = Vec::new();
    let mut indices = Vec::new();

    for model in &models {
        let mesh = &model.mesh;

        for index in &mesh.indices {
            let i = *index as usize;

            let pos_offset = 3 * i;
            let tex_coord_offset = 2 * i;
            let normal_offset = 3 * i;

            let tex_coord = if mesh.texcoords.is_empty() {
                vec2(0.0, 0.0)
            } else {
                vec2(
                    mesh.texcoords[tex_coord_offset],
                    1.0 - mesh.texcoords[tex_coord_offset + 1],
                )
            };

            let normal = if mesh.normals.is_empty() {
                vec3(0.0, 0.0, 1.0)
            } else {
                vec3(
                    mesh.normals[normal_offset],
                    mesh.normals[normal_offset + 1],
                    mesh.normals[normal_offset + 2],
                )
            };

            let vertex = SourceVertex {
                pos: vec3(
                    mesh.positions[pos_offset],
                    mesh.positions[pos_offset + 1],
                    mesh.positions[pos_offset + 2],
                ),
                color: vec3(1.0, 1.0, 1.0),
                tex_coord,
                normal,
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

    Ok(SourceMesh {
        vertices,
        indices,
        topology: SourceTopology::TriangleList,
    })
}

#[cfg(test)]
mod tests {
    use crate::model::SourceTopology::TriangleList;

    use super::*;
    use cgmath::{vec2, vec3};

    #[test]
    fn to_debugline_data_converts_triangle_indices_to_line_indices() {
        let source = SourceMesh {
            vertices: vec![
                SourceVertex::new(
                    vec3(0.0, 0.0, 0.0),
                    vec3(1.0, 1.0, 1.0),
                    vec2(0.0, 0.0),
                    vec3(0.0, 0.0, 1.0),
                ),
                SourceVertex::new(
                    vec3(1.0, 0.0, 0.0),
                    vec3(1.0, 1.0, 1.0),
                    vec2(1.0, 0.0),
                    vec3(0.0, 0.0, 1.0),
                ),
                SourceVertex::new(
                    vec3(0.0, 1.0, 0.0),
                    vec3(1.0, 1.0, 1.0),
                    vec2(0.0, 1.0),
                    vec3(0.0, 0.0, 1.0),
                ),
            ],
            indices: vec![0, 1, 2],
            topology: SourceTopology::TriangleList,
        };

        let mesh_data = source.to_debugline_data();

        assert_eq!(mesh_data.vertices.len(), 3);
        assert_eq!(mesh_data.indices, vec![0, 1, 1, 2, 2, 0]);
    }
}
