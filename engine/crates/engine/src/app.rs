use anyhow::Result;
use renderer_vulkan::VulkanRenderer;
use winit::event::WindowEvent;
use winit::window::Window;

use super::Input;
use super::Time;
use super::World;

pub struct App {
    pub renderer: VulkanRenderer,
    pub world: World,
    pub input: Input,
    pub time: Time,
}

impl App {
    pub unsafe fn create(window: &Window) -> Result<Self> {
        let renderer = VulkanRenderer::create(window)?;
        Ok(Self {
            renderer,
            world: World::default(),
            input: Input::default(),
            time: Time::default(),
        })
    }

    pub fn handle_event(&mut self, event: &WindowEvent) {
        self.input.handle_event(event);
    }

    pub unsafe fn render(&mut self, window: &Window) -> Result<()> {
        self.renderer.render(window)
    }

    pub fn update(&mut self) -> Result<()> {
        self.time.update();
        self.world.update(self.time.delta_seconds())?;
        self.input.clear_transitions();
        Ok(())
    }

    pub unsafe fn destroy(&mut self) {
        self.renderer.destroy();
    }
}
