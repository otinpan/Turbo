mod buffer;
mod command;
mod device;
mod index;
mod instance;
mod pipeline;
mod swapchain;
mod sync;
mod types;
mod uniform;
mod vertex;
mod model;
mod image;
mod mesh;

use anyhow::{Result, anyhow};
use cgmath::{Matrix4, vec3};
use std::time::Instant;
use vulkanalia::prelude::v1_0::*;
use vulkanalia::vk::ExtDebugUtilsExtensionInstanceCommands;
use vulkanalia::vk::KhrSurfaceExtensionInstanceCommands;
use vulkanalia::vk::KhrSwapchainExtensionDeviceCommands;
use vulkanalia::window as vk_window;
use winit::window::Window;
use crate::transform::Transform;

use self::command::{
    create_command_buffers, create_command_pools,
    update_command_buffer,
};
use self::device::{create_logical_device, pick_physical_device};
use self::instance::{VALIDATION_ENABLED, create_entry, create_instance};
use self::pipeline::{create_pipeline, create_render_pass};
use self::swapchain::{create_framebuffers, create_swapchain,create_swapchain_image_views};
use self::sync::{create_render_finished_semaphores, create_sync_objects};
use self::types::{
    VulkanData,
    Mesh,
    RenderObject,
};
use self::uniform::{
    create_descriptor_pool, create_descriptor_set_layout, create_descriptor_sets,
    create_uniform_buffers, update_uniform_buffer,
};
use self::image::{
    create_texture_image,
    create_texture_image_view,
    create_texture_sampler,
    create_depth_objects,
    create_color_objects,
};
use self::model::{MeshData,load_model};
use self::mesh::{
    create_mesh,
};




pub const MAX_FRAMES_IN_FLIGHT: usize = 2;
type Mat4 = Matrix4<f32>;

pub struct VulkanRenderer {
    entry: Entry,
    instance: Instance,
    pub data: VulkanData,
    device: Device,
    frame: usize,
    pub resized: bool,
    // timer
    pub start: Instant,
    // model count
    pub visible_object_count: usize,
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
        create_swapchain_image_views(&device, &mut data)?;
        create_render_pass(&instance, &device, &mut data)?;
        create_descriptor_set_layout(&device, &mut data)?;
        create_pipeline(&device, &mut data)?;
        create_command_pools(&instance, &device, &mut data)?;
        create_color_objects(&instance,&device,&mut data)?;
        create_depth_objects(&instance,&device,&mut data)?;
        create_framebuffers(&device, &mut data)?;
        create_texture_image(&instance,&device,&mut data)?;
        create_texture_image_view(&device,&mut data)?;
        create_texture_sampler(&device,&mut data)?;
        let mesh_data: MeshData=load_model("src/assets/viking_room.obj")?;
        let mesh: Mesh=create_mesh(&instance,&device,&data,mesh_data)?;
        data.meshes.push(mesh);
        data.render_objects.push(RenderObject {
            mesh_index: 0,
            transform: Transform{
                position: vec3(0.0, -1.25, 1.0),
                ..Default::default()
            }
        });
        data.render_objects.push(RenderObject {
            mesh_index: 0,
            transform: Transform{
                position: vec3(0.0, 1.25, 1.0),
                ..Default::default()
            }
        });
        data.render_objects.push(RenderObject {
            mesh_index: 0,
            transform: Transform{
                position: vec3(0.0, -1.25, -1.0),
                ..Default::default()
            }
        });
        data.render_objects.push(RenderObject {
            mesh_index: 0,
            transform: Transform{
                position: vec3(0.0, 1.25, -1.0),
                ..Default::default()
            }
        });
        create_uniform_buffers(&instance, &device, &mut data)?;
        create_descriptor_pool(&device, &mut data)?;
        create_descriptor_sets(&device, &mut data)?;
        create_command_buffers(&device, &mut data)?;
        create_sync_objects(&device, &mut data)?;
        Ok(Self {
            entry,
            instance,
            data,
            device,
            frame: 0,
            resized: false,
            start: Instant::now(),
            visible_object_count: 1,
        })
    }

    pub unsafe fn render(&mut self, window: &Window) -> Result<()> {
        // 1. wait for fence
        let in_flight_fence = self.data.in_flight_fences[self.frame];
        self.device
            .wait_for_fences(&[in_flight_fence], true, u64::MAX)?;

        // 2. When get swapchain image, then signal image_available_semaphore
        let result = self.device.acquire_next_image_khr(
            self.data.swapchain,
            u64::MAX,
            self.data.image_available_semaphores[self.frame],
            vk::Fence::null(),
        );

        // 3. if swapchain is invalid because changed window size,
        // recreate swapchain
        let (image_index, acquire_code) = match result {
            Ok((image_index, code)) => (image_index as usize, code),
            Err(vk::ErrorCode::OUT_OF_DATE_KHR) => return self.recreate_swapchain(window),
            Err(e) => return Err(anyhow!(e)),
        };

        // 4. if swapchain image is used wait fence
        let image_in_flight = self.data.images_in_flight[image_index];
        if !image_in_flight.is_null() {
            self.device
                .wait_for_fences(&[image_in_flight], true, u64::MAX)?;
        }

        // 5. record fence which use swapchain image
        self.data.images_in_flight[image_index] = in_flight_fence;

        // if image_index is updated from render function,
        // uniform_buffer[index] is updated
        // and then reflect shader via descriptor sets which is binding with pipeline
        // uniform buffer <-> descriptor set <-> pipeline layout <-> pipeline <-> shader
        update_command_buffer(self,image_index)?;
        update_uniform_buffer(self, image_index)?;

        // 6. wait image_available_semaphores
        let wait_semaphores = &[self.data.image_available_semaphores[self.frame]];
        let wait_stages = &[vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT];
        // 7. copy using command buffer
        let command_buffers = &[self.data.command_buffers[image_index]];
        // Presentation can outlive the frame fence, so keep one signal
        // semaphore per swapchain image and only reuse it after that image is
        // acquired again.
        // 8. signal to render_finished_semaphore
        let signal_semaphores = &[self.data.render_finished_semaphores[image_index]];
        let submit_info = vk::SubmitInfo::builder()
            .wait_semaphores(wait_semaphores)
            .wait_dst_stage_mask(wait_stages)
            .command_buffers(command_buffers)
            .signal_semaphores(signal_semaphores);

        // 9. reset fence
        self.device.reset_fences(&[in_flight_fence])?;

        // 10. submit to graphics queue
        self.device
            .queue_submit(self.data.graphics_queue, &[submit_info], in_flight_fence)?;

        let swapchains = &[self.data.swapchain];
        let image_indices = &[image_index as u32];

        // 11. present_queue wait for render_finished_semaphore
        let present_info = vk::PresentInfoKHR::builder()
            .wait_semaphores(signal_semaphores)
            .swapchains(swapchains)
            .image_indices(image_indices);

        // 12. presentation
        let result = self
            .device
            .queue_present_khr(self.data.present_queue, &present_info);
        // 13. if window is resized, recreate swapchain
        let changed = acquire_code == vk::SuccessCode::SUBOPTIMAL_KHR
            || result == Ok(vk::SuccessCode::SUBOPTIMAL_KHR)
            || result == Err(vk::ErrorCode::OUT_OF_DATE_KHR);
        if self.resized || changed {
            self.resized = false;
            self.recreate_swapchain(window)?;
        } else if let Err(e) = result {
            return Err(anyhow!(e));
        }

        self.frame = (self.frame + 1) % MAX_FRAMES_IN_FLIGHT;

        Ok(())
    }

    unsafe fn recreate_swapchain(&mut self, window: &Window) -> Result<()> {
        self.device.device_wait_idle()?;
        self.destroy_swapchain();
        create_swapchain(window, &self.instance, &self.device, &mut self.data)?;
        create_swapchain_image_views(&self.device, &mut self.data)?;
        create_render_pass(&self.instance, &self.device, &mut self.data)?;
        create_pipeline(&self.device, &mut self.data)?;
        create_depth_objects(&self.instance, &self.device, &mut self.data)?;
        create_framebuffers(&self.device, &mut self.data)?;
        create_uniform_buffers(&self.instance, &self.device, &mut self.data)?;
        create_descriptor_pool(&self.device, &mut self.data)?;
        create_descriptor_sets(&self.device, &mut self.data)?;
        create_command_buffers(&self.device, &mut self.data)?;
        create_render_finished_semaphores(&self.device, &mut self.data)?;
        self.data.images_in_flight = vec![vk::Fence::null(); self.data.swapchain_images.len()];
        Ok(())
    }

    pub unsafe fn destroy(&mut self) {
        self.device.device_wait_idle().unwrap();


        self.device.destroy_image_view(self.data.color_image_view,None);
        self.device.free_memory(self.data.color_image_memory,None);
        self.device.destroy_image(self.data.color_image,None);

        self.destroy_swapchain();
        
        self.data.command_pools
            .iter()
            .for_each(|p| self.device.destroy_command_pool(*p,None));

        self.device.destroy_sampler(self.data.texture_sampler,None);

        self.device.destroy_image_view(self.data.texture_image_view,None);

        self.device.destroy_image(self.data.texture_image,None);
        self.device.free_memory(self.data.texture_image_memory,None);
        self.device
            .destroy_descriptor_set_layout(self.data.descriptor_set_layout, None);

        self.data
            .in_flight_fences
            .iter()
            .for_each(|f| self.device.destroy_fence(*f, None));
        self.data
            .render_finished_semaphores
            .iter()
            .for_each(|s| self.device.destroy_semaphore(*s, None));
        self.data
            .image_available_semaphores
            .iter()
            .for_each(|s| self.device.destroy_semaphore(*s, None));
        self.data.meshes.drain(..).for_each(|mesh| {
            self.device.free_memory(mesh.index_buffer_memory, None);
            self.device.destroy_buffer(mesh.index_buffer, None);
            self.device.free_memory(mesh.vertex_buffer_memory, None);
            self.device.destroy_buffer(mesh.vertex_buffer, None);
        });
        self.device
            .destroy_command_pool(self.data.command_pool, None);
        self.device.destroy_device(None);
        self.instance.destroy_surface_khr(self.data.surface, None);

        if VALIDATION_ENABLED {
            self.instance
                .destroy_debug_utils_messenger_ext(self.data.messenger, None);
        }

        self.instance.destroy_instance(None);
    }

    unsafe fn destroy_swapchain(&mut self) {
        self.data.command_buffers.clear();
        self.device
            .destroy_descriptor_pool(self.data.descriptor_pool, None);
        self.data
            .uniform_buffers
            .drain(..)
            .for_each(|b| self.device.destroy_buffer(b, None));
        self.data
            .uniform_buffers_memory
            .drain(..)
            .for_each(|m| self.device.free_memory(m, None));
        self.data.descriptor_sets.clear();
        self.data
            .render_finished_semaphores
            .drain(..)
            .for_each(|s| self.device.destroy_semaphore(s, None));
        self.data
            .framebuffers
            .drain(..)
            .for_each(|f| self.device.destroy_framebuffer(f, None));
        self.device.destroy_image_view(self.data.depth_image_view, None);
        self.device.destroy_image(self.data.depth_image, None);
        self.device.free_memory(self.data.depth_image_memory, None);
        self.device.destroy_pipeline(self.data.pipeline, None);
        self.device
            .destroy_pipeline_layout(self.data.pipeline_layout, None);
        self.device.destroy_render_pass(self.data.render_pass, None);
        self.data
            .swapchain_image_views
            .iter()
            .for_each(|v| self.device.destroy_image_view(*v, None));
        self.device.destroy_swapchain_khr(self.data.swapchain, None);
    }
}
