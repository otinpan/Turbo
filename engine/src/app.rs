use anyhow::Result;
use winit::window::Window;

use crate::vulkan::VulkanRenderer;

pub struct App {
    pub renderer: VulkanRenderer,
}

impl App {
    pub unsafe fn create(window: &Window) -> Result<Self> {
        let renderer = VulkanRenderer::create(window)?;
        Ok(Self { renderer })
    }

    pub unsafe fn render(&mut self, window: &Window) -> Result<()> {
        self.renderer.render(window)
    }

    pub unsafe fn destroy(&mut self) {
        self.renderer.destroy();
    }
}
