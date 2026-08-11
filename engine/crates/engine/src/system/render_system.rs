use anyhow::{Result};
use cgmath::{vec3};
use crate::{Registry};
use renderer_vulkan::{RenderItem,RenderCamera,VulkanRenderer};

pub struct RenderSystem;

impl RenderSystem{
    pub fn update(&mut self, registry: &mut Registry, renderer: &mut VulkanRenderer) -> Result<()>{
        let render_items: Vec<RenderItem>=registry
            .renderables()
            .map(|renderable|{
                let mesh_renderer=renderable.mesh_renderer;
                RenderItem {
                    mesh_index: mesh_renderer.mesh,
                    transform: renderable.transform.clone(),
                    alpha: mesh_renderer.material.alpha,
                    material_color: mesh_renderer.material.color,
                    use_texture: mesh_renderer.material.use_texture,
                    texture_index: mesh_renderer.material.texture,
                    pipeline_key: mesh_renderer.material.pipeline_key,
                    is_visible: renderable
                        .visibility
                        .is_none_or(|visibility| visibility.is_visible),
                }
            })
            .collect();

        renderer.set_render_items(render_items);

        if let Some(camera_entity)=registry.active_camera_entity(){
            if let (Some(camera),Some(transform))=(
                registry.camera(camera_entity),
                registry.transform(camera_entity),
            ){
                renderer.set_camera(
                    RenderCamera{
                        position: transform.position,
                        target: camera.target,
                        up: vec3(0.0,0.0,1.0),
                        fov_y: camera.fov_y,
                        near: camera.near,
                        far: camera.far,
                    }
                );
            }
        }
        Ok(())
    }

} 