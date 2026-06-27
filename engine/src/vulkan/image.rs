use std::fs::File;
use anyhow::{Result,anyhow};
use std::ptr::copy_nonoverlapping as memcpy;
use vulkanalia::prelude::v1_0::*;

use super::types::VulkanData;
use super::Instance;
use super::Device;
use super::buffer::{create_buffer,begin_single_time_commands,end_single_time_commands};

pub unsafe fn create_texture_image(
  instance: &Instance,
  device: &Device,
  data: &mut VulkanData,
) -> Result<()>{
  // load
  let image=File::open("../assets/texture.png")?;

  let decoder=png::Decoder::new(image);
  let mut reader=decoder.read_info()?;

  let mut pixels=vec![0;reader.info().raw_bytes()];
  reader.next_frame(&mut pixels)?;

  let size=reader.info().raw_bytes() as u64;
  let (width,height)=reader.info().size();

  // create (staging)
  let (staging_buffer,staging_buffer_memory)=create_buffer(
    instance,
    device,
    data,
    size,
    vk::BufferUsageFlags::TRANSFER_SRC,
    vk::MemoryPropertyFlags::HOST_COHERENT | vk::MemoryPropertyFlags::HOST_VISIBLE,
  )?;

  // copy (staging)
  let  memory=device.map_memory(
    staging_buffer_memory,
    0,
    size,
    vk::MemoryMapFlags::empty()
  )?;
  memcpy(pixels.as_ptr(),memory.cast(),pixels.len());
  device.unmap_memory(staging_buffer_memory);

  let (texture_image,texture_image_memory)=create_image(
    instance,
    device,
    data,
    width,
    height,
    vk::Format::R8G8B8A8_SRGB,
    vk::ImageTiling::OPTIMAL,
    vk::ImageUsageFlags::SAMPLED | vk::ImageUsageFlags::TRANSFER_DST,
    vk::MemoryPropertyFlags::DEVICE_LOCAL,
  )?;

  data.texture_image=texture_image;
  data.texture_image_memory=texture_image_memory;

  // transition + copy
  // change layout to TRANSFER_DST_OPTIMAL to write staging buffer to texture image
  transition_image_layout(
    device,
    data,
    data.texture_image,
    vk::Format::R8G8B8A8_SRGB,
    vk::ImageLayout::UNDEFINED,
    vk::ImageLayout::TRANSFER_DST_OPTIMAL,
  )?;

  // copy pixel data from staging buffer to texture image
  copy_buffer_to_image(device,data,staging_buffer,data.texture_image,width,height)?;

  // change image layout to SHADER_READ_ONLY_OPTIMAL to read from shader
  transition_image_layout(
    device,
    data,
    data.texture_image,
    vk::Format::R8G8B8A8_SRGB,
    vk::ImageLayout::TRANSFER_DST_OPTIMAL,
    vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL,
  )?;

  // clean up
  device.destroy_buffer(staging_buffer,None);
  device.free_memory(staging_buffer_memory,None);

  Ok(())
}

unsafe fn create_image(
  instance: &Instance,
  device: &Device,
  data: &VulkanData,
  width: u32,
  height: u32,
  format: vk::Format,
  tiling: vk::ImageTiling,
  usage: vk::ImageUsageFlags,
  properties: vk::MemoryPropertyFlags,
) -> Result<(vk::Image,vk::DeviceMemory)>{
  // instraction what type of image
  let info=vk::ImageCreateInfo::builder()
    .image_type(vk::ImageType::_2D)
    .extent(vk::Extent3D{
      width,
      height,
      depth: 1,
    })
    .mip_levels(1)
    .array_layers(1)
    .format(vk::Format::R8G8B8A8_SRGB)
    .tiling(vk::ImageTiling::OPTIMAL)
    .initial_layout(vk::ImageLayout::UNDEFINED)
    .usage(vk::ImageUsageFlags::SAMPLED | vk::ImageUsageFlags::TRANSFER_DST)
    .sharing_mode(vk::SharingMode::EXCLUSIVE)
    .samples(vk::SampleCountFlags::_1)
    .flags(vk::ImageCreateFlags::empty());

  // create image, but not allocated in GPU memory
  let image=device.create_image(&info,None)?;
  // get memory size
  let requirements=device.get_image_memory_requirements(image);

  // search GPU memory which is DEVICE_LOCAL and usable this image
  let info=vk::MemoryAllocateInfo::builder()
    .allocation_size(requirements.size)
    .memory_type_index(get_memory_type_index(
      instance,
      data,
      vk::MemoryPropertyFlags::DEVICE_LOCAL,
      requirements,
    )?);

    // allocate
    let image_memory=device.allocate_memory(&info,None)?;

    // bind
    device.bind_image_memory(image,image_memory,0)?;

    Ok((image,image_memory))
}

unsafe fn get_memory_type_index(
    instance: &Instance,
    data: &VulkanData,
    properties: vk::MemoryPropertyFlags,
    requirements: vk::MemoryRequirements,
) -> Result<u32> {
    let memory = instance.get_physical_device_memory_properties(data.physical_device);
    (0..memory.memory_type_count)
        .find(|i| {
            let suitable = (requirements.memory_type_bits & (1 << i)) != 0;
            let memory_type = memory.memory_types[*i as usize];
            suitable && memory_type.property_flags.contains(properties)
        })
        .ok_or_else(|| anyhow!("Failed to find suitable memory type."))
}


// change image layout
unsafe fn transition_image_layout(
  device: &Device,
  data: &VulkanData,
  image: vk::Image,
  format: vk::Format,
  old_layout: vk::ImageLayout,
  new_layout: vk::ImageLayout,
) -> Result<()>{
  let(
    src_access_mask,
    dst_access_mask,
    src_stage_mask,
    dst_stage_mask
  )=match(old_layout,new_layout){
    (vk::ImageLayout::UNDEFINED,vk::ImageLayout::TRANSFER_DST_OPTIMAL) =>(
      vk::AccessFlags::empty(),
      vk::AccessFlags::TRANSFER_WRITE,
      vk::PipelineStageFlags::TOP_OF_PIPE,
      vk::PipelineStageFlags::TRANSFER,
    ),
    (vk::ImageLayout::TRANSFER_DST_OPTIMAL,vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL) =>(
      vk::AccessFlags::TRANSFER_WRITE,
      vk::AccessFlags::SHADER_READ,
      vk::PipelineStageFlags::TRANSFER,
      vk::PipelineStageFlags::FRAGMENT_SHADER,
    ),
    _ => return Err(anyhow!("Unsupported image layout transition!")),
  };

  let command_buffer=begin_single_time_commands(device,data)?;

  let subresource=vk::ImageSubresourceRange::builder()
    .aspect_mask(vk::ImageAspectFlags::COLOR)
    .base_mip_level(0)
    .level_count(1)
    .base_array_layer(0)
    .layer_count(1);

  let barrier=vk::ImageMemoryBarrier::builder()
    .old_layout(old_layout)
    .new_layout(new_layout)
    .src_queue_family_index(vk::QUEUE_FAMILY_IGNORED)
    .dst_queue_family_index(vk::QUEUE_FAMILY_IGNORED)
    .image(image)
    .subresource_range(subresource)
    .src_access_mask(src_access_mask)
    .dst_access_mask(dst_access_mask);

  device.cmd_pipeline_barrier(
    command_buffer,
    src_stage_mask,
    dst_stage_mask,
    vk::DependencyFlags::empty(),
    &[] as &[vk::MemoryBarrier],
    &[] as &[vk::BufferMemoryBarrier],
    &[barrier],
  );


  end_single_time_commands(device,data,command_buffer)?;

  Ok(())
}



// copy image data in staging buffer to gpu 
unsafe fn copy_buffer_to_image(
  device: &Device,
  data: &VulkanData,
  buffer: vk::Buffer,
  image: vk::Image,
  width: u32,
  height: u32,
) -> Result<()>{
  // create command_buffer
  let command_buffer=begin_single_time_commands(device,data)?;

  let subresource = vk::ImageSubresourceLayers::builder()
    .aspect_mask(vk::ImageAspectFlags::COLOR)
    .mip_level(0)
    .base_array_layer(0)
    .layer_count(1);

  let region = vk::BufferImageCopy::builder()
    .buffer_offset(0)
    .buffer_row_length(0)
    .buffer_image_height(0)
    .image_subresource(subresource)
    .image_offset(vk::Offset3D { x: 0, y: 0, z: 0 })
    .image_extent(vk::Extent3D {
        width,
        height,
        depth: 1,
    });

  // write copy instruction to command_buffer
  // copy image from staging buffer to gpt image memory
  device.cmd_copy_buffer_to_image(
    command_buffer,
    buffer,
    image,
    vk::ImageLayout::TRANSFER_DST_OPTIMAL,
    &[region],
  );

  end_single_time_commands(device,data,command_buffer)?;
  Ok(())
}