use anyhow::Result;
use cgmath::vec3;
use renderer_vulkan::{RenderItem, VulkanRenderer};
use turbo_math::Transform;
use winit::event::WindowEvent;
use winit::keyboard::KeyCode;
use winit::window::Window;

use super::Input;
use super::MeshHandle;
use super::Time;
use super::World;

pub type Vec3=cgmath::Vector3<f32>;

pub struct App {
    pub renderer: VulkanRenderer,
    pub world: World,
    pub input: Input,
    pub time: Time,
    mesh: MeshHandle,
    positions: Vec<Vec3>,
}

impl App {
    pub unsafe fn create(window: &Window) -> Result<Self> {
        let mut renderer = VulkanRenderer::create(window)?;
        let mut world = World::default();

        let mesh = MeshHandle(renderer.load_mesh("assets/models/viking_room.obj")?);
        let mut positions: Vec<Vec3>=vec![];
        positions.push(vec3(0.0, -1.25, 1.0));
        positions.push(vec3(0.0,1.25,1.0));
        positions.push(vec3(0.0,-1.25,-1.0));
        positions.push(vec3(0.0,1.25,-1.0));

        let mut app = Self {
            renderer,
            world,
            input: Input::default(),
            time: Time::default(),
            mesh,
            positions,
        };
        app.sync_renderer();

        Ok(app)
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

        if self.input.key_pressed(KeyCode::ArrowLeft) {
            let id=self.world.objects().last()
                .map(|object| object.id);

            if let Some(id) = id{
                self.world.despawn(id);
            }
        }
        if self.input.key_pressed(KeyCode::ArrowRight) {
            if self.positions.len()>self.world.objects().len(){
                let id=self.world.spawn(
                    self.mesh,
                    Transform {
                        position: self.positions[self.world.objects().len()],
                        ..Default::default()
                    }
                );
            }

        }
        self.sync_renderer();
        self.input.clear_transitions();
        Ok(())
    }


    pub unsafe fn destroy(&mut self) {
        self.renderer.destroy();
    }

    fn sync_renderer(&mut self) {
        let render_items = self
            .world
            .objects()
            .iter()
            .map(|object| RenderItem {
                mesh_index: object.mesh.0,
                transform: object.transform.clone(),
            })
            .collect();

        self.renderer.set_render_items(render_items);
    }
}
