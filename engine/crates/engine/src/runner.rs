use anyhow::Result;
use winit::dpi::LogicalSize;
use winit::event::{Event, WindowEvent};
use winit::event_loop::EventLoop;
use winit::window::WindowBuilder;

use crate::App;

pub struct EngineConfig {
    pub title: String,
    pub width: u32,
    pub height: u32,
}

impl Default for EngineConfig {
    fn default() -> Self {
        Self {
            title: "Turbo Engine".to_string(),
            width: 1024,
            height: 768,
        }
    }
}

pub fn run<F>(config: EngineConfig, setup: F) -> Result<()>
where
    F: FnOnce(&mut App) -> Result<()>,
{
    let event_loop = EventLoop::new()?;
    let window = WindowBuilder::new()
        .with_title(config.title)
        .with_inner_size(LogicalSize::new(config.width, config.height))
        .build(&event_loop)?;

    let mut app = unsafe { App::create(&window)? };
    setup(&mut app)?;

    let mut minimized = false;
    event_loop.run(move |event, elwt| match event {
        Event::AboutToWait => window.request_redraw(),
        Event::WindowEvent { event, .. } => {
            app.handle_event(&event);

            match event {
                WindowEvent::RedrawRequested if !elwt.exiting() && !minimized => unsafe {
                    app.update().unwrap();
                    app.render(&window).unwrap();
                },
                WindowEvent::Resized(size) => {
                    if size.width == 0 || size.height == 0 {
                        minimized = true;
                    } else {
                        minimized = false;
                        app.renderer.resized = true;
                    }
                }
                WindowEvent::CloseRequested => {
                    elwt.exit();
                    unsafe {
                        app.destroy();
                    }
                }
                _ => {}
            }
        }
        _ => {}
    })?;

    Ok(())
}
