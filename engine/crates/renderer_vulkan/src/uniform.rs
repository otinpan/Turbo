use anyhow::Result;
use cgmath::num_traits::clamp_max;
use cgmath::{Deg, point3, vec3};
use std::mem::size_of;
use std::ptr::copy_nonoverlapping as memcpy;
use vulkanalia::prelude::v1_0::*;

use super::VulkanRenderer;
use super::buffer::create_buffer;
use super::types::VulkanData;

type Mat4 = cgmath::Matrix4<f32>;

#[repr(C)]
#[derive(Copy, Clone, Debug)]
struct UniformBufferObject {
    view: Mat4,
    proj: Mat4,
}

pub unsafe fn create_descriptor_set_layout(device: &Device, data: &mut VulkanData) -> Result<()> {
    let ubo_binding = vk::DescriptorSetLayoutBinding::builder()
        .binding(0)
        .descriptor_type(vk::DescriptorType::UNIFORM_BUFFER)
        .descriptor_count(1)
        .stage_flags(vk::ShaderStageFlags::VERTEX);

    let sampler_binding = vk::DescriptorSetLayoutBinding::builder()
        .binding(1)
        .descriptor_type(vk::DescriptorType::COMBINED_IMAGE_SAMPLER)
        .descriptor_count(1)
        .stage_flags(vk::ShaderStageFlags::FRAGMENT);

    let bindings = &[ubo_binding, sampler_binding];
    let info = vk::DescriptorSetLayoutCreateInfo::builder().bindings(bindings);

    data.descriptor_set_layout = device.create_descriptor_set_layout(&info, None)?;

    Ok(())
}

pub unsafe fn create_uniform_buffers(
    instance: &Instance,
    device: &Device,
    data: &mut VulkanData,
) -> Result<()> {
    data.uniform_buffers.clear();
    data.uniform_buffers_memory.clear();

    for _ in 0..data.swapchain_images.len() {
        let (uniform_buffer, uniform_buffer_memory) = create_buffer(
            instance,
            device,
            data,
            size_of::<UniformBufferObject>() as u64,
            vk::BufferUsageFlags::UNIFORM_BUFFER,
            vk::MemoryPropertyFlags::HOST_COHERENT | vk::MemoryPropertyFlags::HOST_VISIBLE,
        )?;

        data.uniform_buffers.push(uniform_buffer);
        data.uniform_buffers_memory.push(uniform_buffer_memory);
    }

    Ok(())
}

// create pool to record descriptor_set
pub unsafe fn create_descriptor_pool(device: &Device, data: &mut VulkanData) -> Result<()> {
    let ubo_size = vk::DescriptorPoolSize::builder()
        .type_(vk::DescriptorType::UNIFORM_BUFFER)
        .descriptor_count(data.swapchain_images.len() as u32);

    let sampler_size = vk::DescriptorPoolSize::builder()
        .type_(vk::DescriptorType::COMBINED_IMAGE_SAMPLER)
        .descriptor_count(data.swapchain_images.len() as u32);

    let pool_sizes = &[ubo_size, sampler_size];
    let info = vk::DescriptorPoolCreateInfo::builder()
        .pool_sizes(pool_sizes)
        .max_sets(data.swapchain_images.len() as u32);

    data.descriptor_pool = device.create_descriptor_pool(&info, None)?;

    Ok(())
}

// if image_index is updated from render function,
// uniform_buffer[index] is updated
// and then reflect shader via descriptor sets which is binding with pipeline
// uniform buffer <-> descriptor set <-> pipeline layout <-> pipeline <-> shader
pub unsafe fn create_descriptor_sets(device: &Device, data: &mut VulkanData) -> Result<()> {
    let layouts = vec![data.descriptor_set_layout; data.swapchain_images.len()];
    let info = vk::DescriptorSetAllocateInfo::builder()
        .descriptor_pool(data.descriptor_pool)
        .set_layouts(&layouts);

    // allocate memory for descriptor set.
    data.descriptor_sets = device.allocate_descriptor_sets(&info)?;

    for i in 0..data.swapchain_images.len() {
        let info = vk::DescriptorBufferInfo::builder()
            .buffer(data.uniform_buffers[i])
            .offset(0)
            .range(size_of::<UniformBufferObject>() as u64);

        // this descirptor use unifrom_buffers[i].
        let buffer_info = &[info];
        let ubo_write = vk::WriteDescriptorSet::builder()
            .dst_set(data.descriptor_sets[i])
            .dst_binding(0)
            .dst_array_element(0)
            .descriptor_type(vk::DescriptorType::UNIFORM_BUFFER)
            .buffer_info(buffer_info);

        // sampler
        let info = vk::DescriptorImageInfo::builder()
            .image_layout(vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL)
            .image_view(data.texture_image_view)
            .sampler(data.texture_sampler);

        let image_info = &[info];
        let sampler_write = vk::WriteDescriptorSet::builder()
            .dst_set(data.descriptor_sets[i])
            .dst_binding(1)
            .dst_array_element(0)
            .descriptor_type(vk::DescriptorType::COMBINED_IMAGE_SAMPLER)
            .image_info(image_info);

        device.update_descriptor_sets(&[ubo_write, sampler_write], &[] as &[vk::CopyDescriptorSet]);
    }

    Ok(())
}

pub unsafe fn update_uniform_buffer(
    renderer: &mut VulkanRenderer,
    image_index: usize,
) -> Result<()> {
    // crate camera
    let camera=&renderer.data.camera;

    let view=Mat4::look_at_rh(
        point3(camera.position.x,camera.position.y,camera.position.z),
        point3(camera.target.x,camera.target.y,camera.target.z),
        camera.up,
    );

    #[rustfmt::skip]
    let correction=Mat4::new(
        1.0,0.0,0.0,0.0,
        0.0,-1.0,0.0,0.0,
        0.0,0.0,1.0/2.0,0.0,
        0.0,0.0,1.0/2.0,1.0,
    );

    // how to project
    // when far from camera, then objects are shown smaller.
    let proj=correction
        * cgmath::perspective(
            Deg(camera.fov_y),
            renderer.data.swapchain_extent.width as f32
                / renderer.data.swapchain_extent.height as f32,
            camera.near,
            camera.far,
        );

    let ubo = UniformBufferObject { view, proj };

    // map gpu memory
    let memory = renderer.device.map_memory(
        renderer.data.uniform_buffers_memory[image_index],
        0,
        size_of::<UniformBufferObject>() as u64,
        vk::MemoryMapFlags::empty(),
    )?;

    // copy ubo to memory
    memcpy(&ubo, memory.cast(), 1);
    renderer
        .device
        .unmap_memory(renderer.data.uniform_buffers_memory[image_index]);
    Ok(())
}
