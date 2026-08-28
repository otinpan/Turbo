#![allow(
    dead_code,
    unsafe_op_in_unsafe_fn,
    unused_variables,
    clippy::too_many_arguments,
    clippy::unnecessary_wraps
)]

mod basic_3d_scene;

use anyhow::Result;
use cgmath::{Vector3, vec3};
use turbo_engine::{App, Command, CommandContext, InputTrigger, ObjectApi, PipelineKey};
use winit::dpi::LogicalSize;
use winit::event::{Event, WindowEvent};
use winit::event_loop::EventLoop;
use winit::keyboard::KeyCode;
use winit::window::WindowBuilder;

type Vec3 = Vector3<f32>;

#[derive(Clone, Debug)]
struct CreateTriangle {
    p0: Vec3,
    p1: Vec3,
    p2: Vec3,
    color: Vec3,
    alpha: f32,
    texture: Option<&'static str>,
    pipeline_key: PipelineKey,
}

impl Command for CreateTriangle {
    fn id(&self) -> String {
        "example_create_triangle".to_string()
    }

    fn execute(&self, context: &mut CommandContext<'_>) -> Result<()> {
        context.spawn_triangle_3d(
            self.p0,
            self.p1,
            self.p2,
            self.color,
            self.alpha,
            self.texture,
            self.pipeline_key,
        )?;

        Ok(())
    }
}

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
    app.bind_key(
        KeyCode::Digit3,
        InputTrigger::Pressed,
        CreateTriangle {
            p0: vec3(0.0, 2.0, -0.3),
            p1: vec3(-7.0, 2.0, 0.3),
            p2: vec3(-2.0, 2.0, 1.0),
            color: vec3(1.0, 1.0, 0.0),
            alpha: 1.0,
            texture: Some("face"),
            pipeline_key: PipelineKey::Lit3D,
        },
    );
    let mut minimized = false;
    event_loop.run(move |event, elwt| {
        match event {
            // Request a redraw when all events were processed.
            Event::AboutToWait => window.request_redraw(),
            Event::WindowEvent { event, .. } => {
                app.handle_event(&event);

                match event {
                    // Render a frame if our Vulkan app is not being destroyed.
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
                    // Destroy our Vulkan app.
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
        }
    })?;

    Ok(())
}
