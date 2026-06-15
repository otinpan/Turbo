mod instance;
mod types;
mod device;

use anyhow::Result;
use vulkanalia::prelude::v1_0::*;
use vulkanalia::vk::ExtDebugUtilsExtensionInstanceCommands;
use vulkanalia::vk::KhrSurfaceExtensionInstanceCommands;
use vulkanalia::window as vk_window;
use winit::window::{Window,WindowBuilder};


use self::device::{create_logical_device, pick_physical_device};
use self::instance::{VALIDATION_ENABLED, create_entry, create_instance};
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
        data.surface=vk_window::create_surface(&instance,&window,&window)?;
        // device
        pick_physical_device(&instance, &mut data)?;
        let device = create_logical_device(&entry, &instance, &mut data)?;
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
        self.device.destroy_device(None);
        if VALIDATION_ENABLED {
            self.instance
                .destroy_debug_utils_messenger_ext(self.data.messenger, None);
        }
        self.instance.destroy_surface_khr(self.data.surface,None);
        self.instance.destroy_instance(None);
    }
}
