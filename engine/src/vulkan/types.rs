use vulkanalia::prelude::v1_0::*;

#[derive(Clone, Debug, Default)]
pub struct VulkanData {
    // Debug
    pub messenger: vk::DebugUtilsMessengerEXT,
    // Physical Device
    pub physical_device: vk::PhysicalDevice,
    pub graphics_queue: vk::Queue,
}
