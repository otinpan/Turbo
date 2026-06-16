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

  // Vertex Input
  let vertex_input_state=vk::PipelineVertexInputStateCreateInfo::builder();

  // Input assembly
  let input_assembly_state=vk::PipelineInputAssemblyStateCreateInfo::builder()
    .topology(vk::PrimitiveTopology::TRIANGLE_LIST)
    .primitive_restart_enable(false);

  // viewport
  let viewport=vk::Viewport::builder()
    .x(0.0)
    .y(0.0)
    .width(data.swapchain_extent.width as f32)
    .height(data.swapchain_extent.height as f32)
    .min_depth(0.0)
    .max_depth(1.0);

  // scissor
  let scissor=vk::Rect2D::builder()
    .offset(vk::Offset2D{x:0,y:0})
    .extent(data.swapchain_extent);

  let viewports=&[viewport];
  let scissors=&[scissor];
  let viewport_state=vk::PipelineViewportStateCreateInfo::builder()
    .viewports(viewports)
    .scissors(scissors);

  // rasterizer
  let rasterization_state=vk::PipelineRasterizationStateCreateInfo::builder()
    .depth_clamp_enable(false)
    .rasterizer_discard_enable(false)
    .polygon_mode(vk::PolygonMode::FILL)
    .line_width(1.0)
    .cull_mode(vk::CullModeFlags::BACK)
    .front_face(vk::FrontFace::CLOCKWISE)
    .depth_bias_enable(false);

  // Multisampling (MSAA off)
  let multisampling_state=vk::PipelineMultisampleStateCreateInfo::builder()
    .sample_shading_enable(false)
    .rasterization_samples(vk::SampleCountFlags::_1);

  // Color Blend State
  let attachment=vk::PipelineColorBlendAttachmentState::builder()
    .color_write_mask(vk::ColorComponentFlags::all())
    .blend_enable(false);

  let attachments=&[attachment];
  let color_blend_state=vk::PipelineColorBlendStateCreateInfo::builder()
    .logic_op_enable(false)
    .logic_op(vk::LogicOp::COPY)
    .attachments(attachments)
    .blend_constants([0.0,0.0,0.0,0.0]);

  // Layout
  let layout_info=vk::PipelineLayoutCreateInfo::builder();
  data.pipeline_layout=device.create_pipeline_layout(&layout_info,None)?;

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
