#![allow(
    dead_code,
    unsafe_op_in_unsafe_fn,
    unused_variables,
    clippy::too_many_arguments,
    clippy::unnecessary_wraps
)]

use anyhow::Result;
use turbo_engine::App;
use winit::dpi::LogicalSize;
use winit::event::{ElementState, Event, WindowEvent};
use winit::event_loop::EventLoop;
use winit::keyboard::{KeyCode, PhysicalKey};
use winit::window::WindowBuilder;

fn main() -> Result<()> {
    pretty_env_logger::init();

    // Window

    let event_loop = EventLoop::new()?;
    let window = WindowBuilder::new()
        .with_title("Vulkan Tutorial (Rust)")
        .with_inner_size(LogicalSize::new(1024, 768))
        .build(&event_loop)?;

    // App

    let mut app = unsafe { App::create(&window)? };
    let mut minimized = false;
    event_loop.run(move |event, elwt| {
        match event {
            // Request a redraw when all events were processed.
            Event::AboutToWait => window.request_redraw(),
            Event::WindowEvent { event, .. } => match event {
                // Render a frame if our Vulkan app is not being destroyed.
                WindowEvent::RedrawRequested if !elwt.exiting() && !minimized => {
                    let current_frame = app.renderer.start.elapsed().as_secs_f32();
                    let delta_time = current_frame - app.last_frame;
                    app.last_frame = current_frame;
                    unsafe {
                        app.update(delta_time).unwrap();
                        app.render(&window).unwrap();
                    }
                }
                WindowEvent::Resized(size) => {
                    if size.width == 0 || size.height == 0 {
                        minimized = true;
                    } else {
                        minimized = false;
                        app.renderer.resized = true;
                    }
                }
                // Destroy our Vulkan app.
                WindowEvent::CloseRequested => {
                    elwt.exit();
                    unsafe {
                        app.destroy();
                    }
                }
                WindowEvent::KeyboardInput { event, .. } => {
                    if event.state == ElementState::Pressed {
                        match event.physical_key {
                            PhysicalKey::Code(KeyCode::ArrowLeft)
                                if app.renderer.visible_object_count > 1 =>
                            {
                                app.renderer.visible_object_count -= 1
                            }
                            PhysicalKey::Code(KeyCode::ArrowRight)
                                if app.renderer.visible_object_count < 4 =>
                            {
                                app.renderer.visible_object_count += 1
                            }
                            _ => {}
                        }
                    }
                }
                _ => {}
            },
            _ => {}
        }
    })?;

    Ok(())
}
