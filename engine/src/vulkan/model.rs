use anyhow::Result;
use vulkanalia::prelude::v1_0::*;
use std::fs::File;
use std::collections::HashMap;
use std::io::BufReader;
use cgmath::{vec2, vec3};

use super::types::VulkanData;

type Vec2 = cgmath::Vector2<f32>;
type Vec3 = cgmath::Vector3<f32>;

use super::vertex::Vertex;

pub fn load_model(data: &mut VulkanData) -> Result<()>{
  let mut reader=BufReader::new(File::open("src/assets/viking_room.obj")?);

  let (models,_)=tobj::load_obj_buf(
    &mut reader,
    &tobj::LoadOptions{triangulate: true, ..Default::default()},
    |_| Ok(Default::default()),
  )?;

  let mut unique_vertices=HashMap::new();

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
        data.indices.push(*index as u32);
      }else{
        let index=data.vertices.len();
        unique_vertices.insert(vertex,index);
        data.vertices.push(vertex);
        data.indices.push(index as u32);
      }
    }
  }
  Ok(())
}