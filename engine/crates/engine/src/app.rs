use anyhow::Result;
use cgmath::vec3;
use renderer_vulkan::VulkanRenderer;
use winit::window::Window;

pub struct App {
    pub renderer: VulkanRenderer,
    pub last_frame: f32,
}

impl App {
    pub unsafe fn create(window: &Window) -> Result<Self> {
        let renderer = VulkanRenderer::create(window)?;
        Ok(Self {
            renderer,
            last_frame: 0.0f32,
        })
    }

    pub unsafe fn render(&mut self, window: &Window) -> Result<()> {
        self.renderer.render(window)
    }

    pub unsafe fn update(&mut self, delta_time: f32) -> Result<()> {
        let rotation_speed = vec3(200.0, 0.0, 0.0);
        for object in &mut self.renderer.data.render_objects {
            object.transform.rotate(rotation_speed * delta_time);
        }
        Ok(())
    }

    pub unsafe fn destroy(&mut self) {
        self.renderer.destroy();
    }
}
