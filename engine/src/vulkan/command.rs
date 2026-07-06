use anyhow::Result;
use vulkanalia::prelude::v1_0::*;
use cgmath::{Deg, vec3};
use super::types::VulkanData;
use super::{device::QueueFamilyIndices};
use super::VulkanRenderer;

type Mat4 = cgmath::Matrix4<f32>;

// command pool ////////////////////////////////////////////////////////////
// created command buffers are pushed into graphics queue in render()
// this command buffer is created at once
pub unsafe fn create_command_pools(
    instance: &Instance,
    device: &Device,
    data: &mut VulkanData,
) -> Result<()> {
    data.command_pool=create_command_pool(instance,device,data)?;

    let num_images=data.swapchain_images.len();
    for _ in 0..num_images{
        let command_pool=create_command_pool(instance,device,data)?;
        data.command_pools.push(command_pool);
    }
    Ok(())
}

unsafe fn create_command_pool(
    instance: &Instance,
    device: &Device,
    data: &mut VulkanData,
) -> Result<vk::CommandPool>{
    let indices = QueueFamilyIndices::get(instance, data, data.physical_device)?;

    let info = vk::CommandPoolCreateInfo::builder()
        .flags(vk::CommandPoolCreateFlags::TRANSIENT)
        .queue_family_index(indices.graphics);

    Ok(device.create_command_pool(&info,None)?)

}


// command buffer /////////////////////////////////////////////////////////////
pub unsafe fn create_command_buffers(device: &Device, data: &mut VulkanData) -> Result<()> {
    let num_images=data.swapchain_images.len();
    for image_index in 0..num_images{
        let allocate_info=vk::CommandBufferAllocateInfo::builder()
            .command_pool(data.command_pools[image_index])
            .level(vk::CommandBufferLevel::PRIMARY)
            .command_buffer_count(1);

        let command_buffer=device.allocate_command_buffers(&allocate_info)?[0];
        data.command_buffers.push(command_buffer);
    }

    data.secondary_command_buffers=vec![vec![];data.swapchain_images.len()];
    Ok(())
}

// update command_buffer per frame
pub unsafe fn update_command_buffer(
    renderer: &mut VulkanRenderer,
    image_index: usize
) -> Result<()> {
    // free memory to avoid memory leak
    let command_pool=renderer.data.command_pools[image_index];
    renderer.device.reset_command_pool(command_pool,vk::CommandPoolResetFlags::empty())?;
    // recreate command_buffer
    let command_buffer=renderer.data.command_buffers[image_index];

    // Commands
    let info=vk::CommandBufferBeginInfo::builder()
        .flags(vk::CommandBufferUsageFlags::ONE_TIME_SUBMIT);
    renderer.device.begin_command_buffer(command_buffer, &info)?;

    // Model
    /*
    let time=renderer.start.elapsed().as_secs_f32();
    let model = Mat4::from_axis_angle(
        vec3(0.0, 0.0, 1.0),
        Deg(90.0)*time
    );

    let model_bytes = std::slice::from_raw_parts(
        &model as *const Mat4 as *const u8,
        size_of::<Mat4>()
    );

    let opacity=0.25f32;
    let opacity_bytes=&opacity.to_ne_bytes()[..];
    */

    let render_area = vk::Rect2D::builder()
        .offset(vk::Offset2D::default())
        .extent(renderer.data.swapchain_extent);

    let color_clear_value = vk::ClearValue {
        color: vk::ClearColorValue {
            float32: [0.0, 0.0, 0.0, 1.0],
        },
    };

    let depth_clear_value = vk::ClearValue {
        depth_stencil: vk::ClearDepthStencilValue { depth: 1.0, stencil: 0 },
    };

    let clear_values = &[color_clear_value, depth_clear_value];


    let info = vk::RenderPassBeginInfo::builder()
        .render_pass(renderer.data.render_pass)
        .framebuffer(renderer.data.framebuffers[image_index])
        .render_area(render_area)
        .clear_values(clear_values);

    renderer.device.cmd_begin_render_pass(
        command_buffer,
        &info,
        vk::SubpassContents::SECONDARY_COMMAND_BUFFERS);

    let draw_count = renderer.visible_object_count.min(renderer.data.render_objects.len());
    let secondary_command_buffers=(0..draw_count)
        .map(|i| update_secondary_command_buffer(renderer, image_index,i))
        .collect::<Result<Vec<_>,_>>()?;

    renderer.device.cmd_execute_commands(command_buffer,&secondary_command_buffers[..]);
    renderer.device.cmd_end_render_pass(command_buffer);

    renderer.device.end_command_buffer(command_buffer)?;

    Ok(())
}

unsafe fn update_secondary_command_buffer(
    renderer: &mut VulkanRenderer,
    image_index: usize,
    model_index: usize,
) -> Result<vk::CommandBuffer>{
    // if secondary_command_buffer (swapchain_images.len()) 
    // is smaller than index, add new vec
    renderer.data.secondary_command_buffers.resize_with(image_index+1,Vec::new);

    let command_buffer={
        let command_buffers=&mut renderer.data.secondary_command_buffers[image_index];

        while model_index >= command_buffers.len(){
            let allocate_info=vk::CommandBufferAllocateInfo::builder()
                .command_pool(renderer.data.command_pools[image_index])
                .level(vk::CommandBufferLevel::SECONDARY)
                .command_buffer_count(1);

            let command_buffer=renderer.device.allocate_command_buffers(&allocate_info)?[0];
            command_buffers.push(command_buffer);
        }

        command_buffers[model_index]
    };

    //  Model
    let object=&renderer.data.render_objects[model_index];
    let mesh=&renderer.data.meshes[object.mesh_index];
    let time=renderer.start.elapsed().as_secs_f32();

    let model=object.transform * Mat4::from_axis_angle(
        vec3(0.0,0.0,1.0),
        Deg(90.0)*time
    );

    let model_bytes=std::slice::from_raw_parts(
        &model as *const Mat4 as *const u8,
        size_of::<Mat4>()
    );

    let opacity=(model_index+1) as f32*0.25;
    let opacity_bytes=&opacity.to_ne_bytes()[..];

    // Unique Info for Secondary Command buffeer
    let inheritance_info=vk::CommandBufferInheritanceInfo::builder()
        .render_pass(renderer.data.render_pass)
        .subpass(0)
        .framebuffer(renderer.data.framebuffers[image_index]);

    let info=vk::CommandBufferBeginInfo::builder()
        .flags(vk::CommandBufferUsageFlags::RENDER_PASS_CONTINUE)
        .inheritance_info(&inheritance_info);

    // Start command buffer
    renderer.device.begin_command_buffer(command_buffer,&info)?;

    renderer.device.cmd_bind_pipeline(command_buffer, vk::PipelineBindPoint::GRAPHICS, renderer.data.pipeline);
    renderer.device.cmd_bind_vertex_buffers(command_buffer, 0, &[mesh.vertex_buffer], &[0]);
    renderer.device.cmd_bind_index_buffer(command_buffer, mesh.index_buffer, 0, vk::IndexType::UINT32);
    renderer.device.cmd_bind_descriptor_sets(
        command_buffer,
        vk::PipelineBindPoint::GRAPHICS,
        renderer.data.pipeline_layout,
        0,
        &[renderer.data.descriptor_sets[image_index]],
        &[],
    );
    renderer.device.cmd_push_constants(
        command_buffer,
        renderer.data.pipeline_layout,
        vk::ShaderStageFlags::VERTEX,
        0,
        model_bytes,
    );
    renderer.device.cmd_push_constants(
        command_buffer,
        renderer.data.pipeline_layout,
        vk::ShaderStageFlags::FRAGMENT,
        64,
        opacity_bytes,
    );
    renderer.device.cmd_draw_indexed(
        command_buffer,
        mesh.index_count,
        1, 0, 0, 0);

    renderer.device.end_command_buffer(command_buffer)?;

    Ok(command_buffer)
}
