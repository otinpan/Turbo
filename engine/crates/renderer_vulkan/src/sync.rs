use anyhow::Result;
use vulkanalia::prelude::v1_0::*;

use super::MAX_FRAMES_IN_FLIGHT;
use super::types::VulkanData;

pub unsafe fn create_sync_objects(device: &Device, data: &mut VulkanData) -> Result<()> {
    let semaphore_info = vk::SemaphoreCreateInfo::builder();
    let fence_info = vk::FenceCreateInfo::builder().flags(vk::FenceCreateFlags::SIGNALED);

    for _ in 0..MAX_FRAMES_IN_FLIGHT {
        data.image_available_semaphores
            .push(device.create_semaphore(&semaphore_info, None)?);

        data.in_flight_fences
            .push(device.create_fence(&fence_info, None)?);
    }

    create_render_finished_semaphores(device, data)?;

    data.images_in_flight = data
        .swapchain_images
        .iter()
        .map(|_| vk::Fence::null())
        .collect();

    Ok(())
}

pub unsafe fn create_render_finished_semaphores(
    device: &Device,
    data: &mut VulkanData,
) -> Result<()> {
    let semaphore_info = vk::SemaphoreCreateInfo::builder();

    // Presentation can outlive a frame fence, so these semaphores belong to
    // swapchain images and must be recreated when the swapchain changes.
    for _ in &data.swapchain_images {
        data.render_finished_semaphores
            .push(device.create_semaphore(&semaphore_info, None)?);
    }

    Ok(())
}
