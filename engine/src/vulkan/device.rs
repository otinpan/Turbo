use anyhow::{anyhow, Result};
use png::DecodingError::IoError;
use std::collections::HashSet;
use log::{info, warn};
use thiserror::Error;
use vulkanalia::prelude::v1_0::*;
use vulkanalia::vk::KhrSurfaceExtensionInstanceCommands;

use super::instance::{PORTABILITY_MACOS_VERSION, VALIDATION_ENABLED, VALIDATION_LAYER};
use super::types::VulkanData;

#[derive(Debug, Error)]
#[error("{0}")]
pub struct SuitabilityError(pub &'static str);

// Physical Device
pub unsafe fn pick_physical_device(
    instance: &Instance,
    data: &mut VulkanData,
) -> Result<()> {
    for physical_device in instance.enumerate_physical_devices()? {
        let properties = instance.get_physical_device_properties(physical_device);

        if let Err(error) = check_physical_device(instance, data, physical_device) {
            warn!(
                "Skipping physical device (`{}`): {}",
                properties.device_name, error
            );
        } else {
            info!("Selected physical device (`{}`).", properties.device_name);
            data.physical_device = physical_device;
            return Ok(());
        }
    }

    Err(anyhow!("Failed to find suitable physical device"))
}

unsafe fn check_physical_device(
    instance: &Instance,
    data: &VulkanData,
    physical_device: vk::PhysicalDevice,
) -> Result<()> {
    QueueFamilyIndices::get(instance, data, physical_device)?;
    Ok(())
}

#[derive(Copy, Clone, Debug)]
struct QueueFamilyIndices {
    graphics: u32,
    present: u32,
}

impl QueueFamilyIndices {
    unsafe fn get(
        instance: &Instance,
        data: &VulkanData,
        physical_device: vk::PhysicalDevice,
    ) -> Result<Self> {
        let properties = instance.get_physical_device_queue_family_properties(physical_device);

        let graphics = properties
            .iter()
            .position(|p| p.queue_flags.contains(vk::QueueFlags::GRAPHICS))
            .map(|i| i as u32);

        let mut present=None;
        for (index,properties) in properties.iter().enumerate(){
            if instance.get_physical_device_surface_support_khr(physical_device,index as u32,data.surface)?{
                present=Some(index as u32);
                break;
            }
        }

        if let (Some(graphics),Some(present)) = (graphics,present) {
            Ok(Self { graphics,present })
        } else {
            Err(anyhow!(SuitabilityError("Missing required queue families")))
        }
    }
}
// Logical Device
pub unsafe fn create_logical_device(
    entry: &Entry,
    instance: &Instance,
    data: &mut VulkanData,
) -> Result<Device> {
    // Queue Create Infos
    let indices = QueueFamilyIndices::get(instance, data, data.physical_device)?;

    let mut unique_indices=HashSet::new();
    unique_indices.insert(indices.graphics);
    unique_indices.insert(indices.present);

    let queue_priorities = &[1.0];
    let queue_infos=unique_indices
        .iter()
        .map(|i|{
            vk::DeviceQueueCreateInfo::builder()
                .queue_family_index(*i)
                .queue_priorities(queue_priorities)
        })
        .collect::<Vec<_>>();

    // Layers
    let layers = if VALIDATION_ENABLED {
        vec![VALIDATION_LAYER.as_ptr()]
    } else {
        vec![]
    };

    // Extensions
    let mut extensions = vec![];

    // Required by Vulkan SDK on macOS since 1.3.216
    if cfg!(target_os = "macos") && entry.version()? >= PORTABILITY_MACOS_VERSION {
        extensions.push(vk::KHR_PORTABILITY_SUBSET_EXTENSION.name.as_ptr());
    }

    // Features
    let features = vk::PhysicalDeviceFeatures::builder();

    // Create
    let info=vk::DeviceCreateInfo::builder()
        .queue_create_infos(&queue_infos)
        .enabled_layer_names(&layers)
        .enabled_extension_names(&extensions)
        .enabled_features(&features);

    let device = instance.create_device(data.physical_device, &info, None)?;

    // Queues
    data.graphics_queue = device.get_device_queue(indices.graphics, 0);
    data.present_queue=device.get_device_queue(indices.present,0);
    Ok(device)
}
