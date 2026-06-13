use vulkanalia::prelude::v1_0::*;

#[derive(Clone, Debug, Default)]
pub struct VulkanData {
    pub messenger: vk::DebugUtilsMessengerEXT,
}
