use vulkanalia::prelude::v1_0::*;

#[derive(Clone, Debug, Default)]
pub struct VulkanData {
    // Debug
    pub messenger: vk::DebugUtilsMessengerEXT,
    // Surface
    pub surface: vk::SurfaceKHR,
    // Physical Device
    pub physical_device: vk::PhysicalDevice,
    pub graphics_queue: vk::Queue,
    pub present_queue: vk::Queue,
    // Swapchain
    pub swapchain_format: vk::Format,
    pub swapchain_extent: vk::Extent2D,
    pub swapchain: vk::SwapchainKHR,
    pub swapchain_images: Vec<vk::Image>,
    // Image views
    pub swapchain_image_views: Vec<vk::ImageView>,
    // Pipeline
    pub render_pass: vk::RenderPass,
    pub pipeline_layout: vk::PipelineLayout,
}
