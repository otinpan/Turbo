use anyhow::Result;
use vulkanalia::prelude::v1_0::*;
use std::fs::File;
use std::collections::HashMap;
use std::hash::{Hash,Hasher};
use std::io::BufReader;
use std::mem::size_of;

use super::VERTICES;
use super::buffer::{copy_buffer, create_buffer};
use super::types::VulkanData;

type Vec2 = cgmath::Vector2<f32>;
type Vec3 = cgmath::Vector3<f32>;

pub fn load_model(data: &mut VulkanData) -> Result<()>{
  let mut reader=BufReader::new(File::open("src/assets/viking_room.ob")?);

  let (models,_)=tobj::load_obj_buf(
    &mut reader,
    &tobj::LoadOptions{triangulate: true, ..Default::default()},
    |_| Ok(Default::default()),
  )?;
  Ok(())
}