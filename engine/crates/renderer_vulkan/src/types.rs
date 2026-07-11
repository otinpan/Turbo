use cgmath::{Matrix4,vec3};
use vulkanalia::prelude::v1_0::*;

use super::vertex::Vertex;
use turbo_math::Transform;

#[derive(Clone, Debug, Default)]
pub struct VulkanData {
    // Debug
    pub messenger: vk::DebugUtilsMessengerEXT,
    // Surface
    pub surface: vk::SurfaceKHR,
    // Physical Device
    pub physical_device: vk::PhysicalDevice,
    pub msaa_samples: vk::SampleCountFlags,
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
    pub descriptor_set_layout: vk::DescriptorSetLayout,
    pub pipeline_layout: vk::PipelineLayout,
    pub pipeline: vk::Pipeline,
    // Descriptors
    pub descriptor_pool: vk::DescriptorPool,
    pub descriptor_sets: Vec<vk::DescriptorSet>,
    // Framebuffers
    pub framebuffers: Vec<vk::Framebuffer>,
    // Command Pool
    pub command_pool: vk::CommandPool,
    // Color
    pub color_image: vk::Image,
    pub color_image_memory: vk::DeviceMemory,
    pub color_image_view: vk::ImageView,
    // Mesh/Object data for the next renderer step.
    pub meshes: Vec<Mesh>,
    pub render_objects: Vec<RenderItem>,
    // Buffers
    pub uniform_buffers: Vec<vk::Buffer>,
    pub uniform_buffers_memory: Vec<vk::DeviceMemory>,
    // Command Buffers
    pub command_pools: Vec<vk::CommandPool>,
    pub command_buffers: Vec<vk::CommandBuffer>,
    pub secondary_command_buffers: Vec<Vec<vk::CommandBuffer>>,
    // Sync Objects
    pub image_available_semaphores: Vec<vk::Semaphore>,
    pub render_finished_semaphores: Vec<vk::Semaphore>,
    pub in_flight_fences: Vec<vk::Fence>,
    pub images_in_flight: Vec<vk::Fence>,
    // Texture Image
    pub mip_levels: u32,
    pub texture_image: vk::Image,
    pub texture_image_memory: vk::DeviceMemory,
    pub texture_image_view: vk::ImageView,
    pub texture_sampler: vk::Sampler,
    // Depth
    pub depth_image: vk::Image,
    pub depth_image_memory: vk::DeviceMemory,
    pub depth_image_view: vk::ImageView,
    // Camera
    pub camera: RenderCamera,
}

#[derive(Clone, Debug)]
pub struct Mesh {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u32>,
    pub vertex_buffer: vk::Buffer,
    pub vertex_buffer_memory: vk::DeviceMemory,
    pub index_buffer: vk::Buffer,
    pub index_buffer_memory: vk::DeviceMemory,
    pub index_count: u32,
}

#[derive(Clone, Debug)]
pub struct RenderItem {
    pub mesh_index: usize,
    pub transform: Transform,
}

// Camera
#[derive(Clone,Debug)]
pub struct RenderCamera{
    pub position: cgmath::Vector3<f32>,
    pub target: cgmath::Vector3<f32>,
    pub up: cgmath::Vector3<f32>,
    pub fov_y: f32,
    pub near: f32,
    pub far: f32,
}

impl Default for RenderCamera{
    fn default() -> Self{
        Self{
            position: vec3(0.0,0.0,-5.0),
            target: vec3(0.0,0.0,0.0),
            up: vec3(0.0,1.0,0.0),
            fov_y: 0.0,
            near: 0.0,
            far: 0.0
        }
    }
}