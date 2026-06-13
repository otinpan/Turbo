mod instance;
mod types;

use anyhow::Result;
use vulkanalia::prelude::v1_0::*;
use vulkanalia::vk::ExtDebugUtilsExtensionInstanceCommands;
use winit::window::Window;

use self::instance::{VALIDATION_ENABLED, create_entry, create_instance};
use self::types::VulkanData;

pub struct VulkanRenderer {
    entry: Entry,
    instance: Instance,
    data: VulkanData,
}

impl VulkanRenderer {
    pub unsafe fn create(window: &Window) -> Result<Self> {
        let entry = create_entry()?;
        let mut data = VulkanData::default();
        let instance = create_instance(window, &entry, &mut data)?;

        Ok(Self {
            entry,
            instance,
            data,
        })
    }

    pub unsafe fn render(&mut self, _window: &Window) -> Result<()> {
        Ok(())
    }

    pub unsafe fn destroy(&mut self) {
        if VALIDATION_ENABLED {
            self.instance
                .destroy_debug_utils_messenger_ext(self.data.messenger, None);
        }
        self.instance.destroy_instance(None);
    }
}
