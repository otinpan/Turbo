use anyhow::Result;
use std::fs::File;
use std::collections::HashMap;
use std::io::BufReader;
use cgmath::{vec2, vec3};

use super::vertex::Vertex;

#[derive(Clone,Debug)]
pub struct MeshData{
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u32>,
}

pub fn load_model(file_path: &str) -> Result<MeshData>{
  let mut reader=BufReader::new(File::open(file_path)?);

  let (models,_)=tobj::load_obj_buf(
    &mut reader,
    &tobj::LoadOptions{triangulate: true, ..Default::default()},
    |_| Ok(Default::default()),
  )?;

  let mut unique_vertices=HashMap::new();
  let mut vertices: Vec<Vertex>=vec![];
  let mut indices: Vec<u32>=vec![];

  for model in &models{
    for index in &model.mesh.indices{
      let pos_offset=(3*index) as usize;
      let tex_coord_offset=(2*index) as usize;

      let vertex=Vertex{
        pos: vec3(
          model.mesh.positions[pos_offset],
          model.mesh.positions[pos_offset+1],
          model.mesh.positions[pos_offset+2],
        ),
        color: vec3(1.0,1.0,1.0),
        tex_coord: vec2(
          model.mesh.texcoords[tex_coord_offset],
          1.0-model.mesh.texcoords[tex_coord_offset+1],
        ),
      };

      if let Some(index)=unique_vertices.get(&vertex){
        indices.push(*index as u32);
      }else{
        let index=vertices.len();
        unique_vertices.insert(vertex,index);
        vertices.push(vertex);
        indices.push(index as u32);
      }
    }
  }
  Ok(MeshData{vertices,indices})
}
