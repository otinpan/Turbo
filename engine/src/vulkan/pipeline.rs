use vulkanalia::bytecode::Bytecode;
use anyhow::{anyhow, Result};
use vulkanalia::prelude::v1_0::*;

use super::instance::{PORTABILITY_MACOS_VERSION, VALIDATION_ENABLED, VALIDATION_LAYER};
use super::types::VulkanData;


pub unsafe fn create_pipeline(device: &Device,data: &mut VulkanData) ->Result<()>{
  // Stages
  let vert=include_bytes!("../shaders/out/vert.spv");
  let frag=include_bytes!("../shaders/out/frag.spv");

  let vert_shader_module=create_shader_module(device,&vert[..])?;
  let frag_shader_module=create_shader_module(device,&frag[..])?;

  let vert_stage=vk::PipelineShaderStageCreateInfo::builder()
    .stage(vk::ShaderStageFlags::VERTEX)
    .module(vert_shader_module)
    .name(b"main\0");

  let frag_stage=vk::PipelineShaderStageCreateInfo::builder()
    .stage(vk::ShaderStageFlags::FRAGMENT)
    .module(frag_shader_module)
    .name(b"main\0");

  // Cleanup
  device.destroy_shader_module(vert_shader_module,None);
  device.destroy_shader_module(frag_shader_module,None);

  Ok(())
}

unsafe fn create_shader_module(device: &Device,bytecode: &[u8]) ->Result<vk::ShaderModule>{
  // convert u8 to u32
  let bytecode=Bytecode::new(bytecode).unwrap();
  let info=vk::ShaderModuleCreateInfo::builder()
    .code(bytecode.code())
    .code_size(bytecode.code_size());
  Ok(device.create_shader_module(&info,None)?)
}