use std::fs::File;
use anyhow::{Result,anyhow};
use std::ptr::copy_nonoverlapping as memcpy;
use vulkanalia::prelude::v1_0::*;

use super::types::VulkanData;
use super::Instance;
use super::Device;
use super::buffer::{create_buffer,begin_single_time_commands,end_single_time_commands};

// texture image ///////////////////////////////////////////////
pub unsafe fn create_texture_image(
  instance: &Instance,
  device: &Device,
  data: &mut VulkanData,
) -> Result<()>{
  // load
  let image=File::open("src/assets/texture.png")?;

  let decoder=png::Decoder::new(image);
  let mut reader=decoder.read_info()?;

  let mut pixels=vec![0;reader.info().raw_bytes()];
  reader.next_frame(&mut pixels)?;

  // if png corresponds RGB, convert to RGBA to add 4bytes
  let pixels=match (reader.info().color_type,reader.info().bit_depth) {
    (png::ColorType::Rgb,png::BitDepth::Eight) => pixels
      .chunks_exact(3)
      .flat_map(|rgb| [rgb[0],rgb[1],rgb[2],255])
      .collect::<Vec<_>>(),
    (png::ColorType::Rgba,png::BitDepth::Eight) => pixels,
    (color_type,bit_depth) => {
      return Err(anyhow!(
        "Unsupported PNG format: {:?} {:?}",
        color_type,
        bit_depth,
      ));
    }
  };

  let size=pixels.len() as u64;
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

  // create image
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
    .format(format)
    .tiling(tiling)
    .initial_layout(vk::ImageLayout::UNDEFINED)
    .usage(usage)
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
      properties,
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


// texture image view //////////////////////////////////////
pub unsafe fn create_texture_image_view(
  device: &Device,
  data: &mut VulkanData,
) -> Result<()>{
  data.texture_image_view=create_image_view(
    device,
    data.texture_image,
    vk::Format::R8G8B8A8_SRGB,
    vk::ImageAspectFlags::COLOR,
  )?;

  Ok(())
}


pub unsafe fn create_image_view(
  device: &Device,
  image: vk::Image,
  format: vk::Format,
  aspects: vk::ImageAspectFlags,
) -> Result<vk::ImageView>{
  let subresource_range=vk::ImageSubresourceRange::builder()
    .aspect_mask(aspects)
    .base_mip_level(0)
    .level_count(1)
    .base_array_layer(0)
    .layer_count(1);

  let info=vk::ImageViewCreateInfo::builder()
    .image(image)
    .view_type(vk::ImageViewType::_2D)
    .format(format)
    .subresource_range(subresource_range);

  Ok(device.create_image_view(&info,None)?)
}

// samper //////////////////////////////////////////////////
pub unsafe fn create_texture_sampler(
  device: &Device,
  data: &mut VulkanData,
) -> Result<()>{
  let info=vk::SamplerCreateInfo::builder()
    .mag_filter(vk::Filter::LINEAR)
    .min_filter(vk::Filter::LINEAR)
    .address_mode_u(vk::SamplerAddressMode::REPEAT) // x
    .address_mode_v(vk::SamplerAddressMode::REPEAT) // y
    .address_mode_w(vk::SamplerAddressMode::REPEAT) // z
    .anisotropy_enable(true)
    .max_anisotropy(16.0)
    .border_color(vk::BorderColor::INT_OPAQUE_BLACK)
    .unnormalized_coordinates(false)
    .compare_enable(false)
    .compare_op(vk::CompareOp::ALWAYS)
    .mipmap_mode(vk::SamplerMipmapMode::LINEAR)
    .mip_lod_bias(0.0)
    .min_lod(0.0)
    .max_lod(0.0);

  data.texture_sampler=device.create_sampler(&info,None)?;
  Ok(())
}

// depth objects /////////////////////////////////////////////////////////
pub unsafe fn create_depth_objects(
  instance: &Instance,
  device: &Device,
  data: &mut VulkanData
) -> Result<()>{
  let format=get_depth_format(instance,data)?;
  
  // create empty depth image
  let (depth_image,depth_image_memory)=create_image(
    instance,
    device,
    data,
    data.swapchain_extent.width,
    data.swapchain_extent.height,
    format,
    vk::ImageTiling::OPTIMAL,
    vk::ImageUsageFlags::DEPTH_STENCIL_ATTACHMENT,
    vk::MemoryPropertyFlags::DEVICE_LOCAL,
  )?;

  data.depth_image=depth_image;
  data.depth_image_memory=depth_image_memory;

  // Image View
  data.depth_image_view=create_image_view(
    device,
    data.depth_image,
    format,
  vk::ImageAspectFlags::DEPTH,
)?;

  Ok(())
}

pub unsafe fn get_depth_format(
  instance: &Instance,
  data: &VulkanData,
) -> Result<vk::Format>{
  let candidates=&[
    vk::Format::D32_SFLOAT,
    vk::Format::D32_SFLOAT_S8_UINT,
    vk::Format::D24_UNORM_S8_UINT,
  ];

  get_supported_format(
    instance,
    data,
    candidates,
    vk::ImageTiling::OPTIMAL,
    vk::FormatFeatureFlags::DEPTH_STENCIL_ATTACHMENT,
  )
}

// take a list of candidate formats in order from most desirable to least desirable
// and returns the first satisfies requirements
unsafe fn get_supported_format(
  instance: &Instance,
  data: &VulkanData,
  candidates: &[vk::Format],
  tiling: vk::ImageTiling,
  features: vk::FormatFeatureFlags,
) -> Result<vk::Format>{
  candidates
    .iter()
    .cloned()
    .find(|f|{
      let properties=instance.get_physical_device_format_properties(
        data.physical_device,
        *f
      );
      match tiling{
        vk::ImageTiling::LINEAR => properties.linear_tiling_features.contains(features),
        vk::ImageTiling::OPTIMAL => properties.optimal_tiling_features.contains(features),
        _ => false,
      }
    })
    .ok_or_else(|| anyhow!("Failed to find supported format!"))
}