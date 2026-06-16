mod instance;
mod types;
mod device;
mod swapchain;
mod pipeline;

use anyhow::Result;
use vulkanalia::prelude::v1_0::*;
use vulkanalia::vk::ExtDebugUtilsExtensionInstanceCommands;
use vulkanalia::vk::KhrSurfaceExtensionInstanceCommands;
use vulkanalia::vk::KhrSwapchainExtensionDeviceCommands;
use vulkanalia::window as vk_window;
use winit::window::Window;


use self::device::{create_logical_device, pick_physical_device};
use self::instance::{VALIDATION_ENABLED, create_entry, create_instance};
use self::swapchain::{create_swapchain,create_swapchain_image_views};
use self::pipeline::{create_pipeline};
use self::types::VulkanData;

pub struct VulkanRenderer {
    entry: Entry,
    instance: Instance,
    data: VulkanData,
    device: Device,
}

impl VulkanRenderer {
    pub unsafe fn create(window: &Window) -> Result<Self> {
        let entry = create_entry()?;
        let mut data = VulkanData::default();
        let instance = create_instance(window, &entry, &mut data)?;
        data.surface = vk_window::create_surface(&instance, &window, &window)?;
        // device
        pick_physical_device(&instance, &mut data)?;
        let device = create_logical_device(&entry, &instance, &mut data)?;
        create_swapchain(window, &instance, &device, &mut data)?;
        create_swapchain_image_views(&device,&mut data)?;
        create_pipeline(&device,&mut data)?;
        Ok(Self {
            entry,
            instance,
            data,
            device,
        })
    }

    pub unsafe fn render(&mut self, _window: &Window) -> Result<()> {
        Ok(())
    }

    pub unsafe fn destroy(&mut self) {
        self.device.destroy_pipeline_layout(self.data.pipeline_layout,None);
        self.data.swapchain_image_views
            .iter()
            .for_each(|v| self.device.destroy_image_view(*v,None));
        self.device.destroy_swapchain_khr(self.data.swapchain, None);
        self.device.destroy_device(None);
        if VALIDATION_ENABLED {
            self.instance
                .destroy_debug_utils_messenger_ext(self.data.messenger, None);
        }
        self.instance.destroy_surface_khr(self.data.surface, None);
        self.instance.destroy_instance(None);
    }
}
