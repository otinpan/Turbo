use anyhow::Result;
use cgmath::vec3;
use renderer_vulkan::{RenderCamera, RenderItem, VulkanRenderer};
use turbo_math::Transform;

use crate::{Camera, MeshRenderer, Registry, Visibility};

#[derive(Clone, Debug)]
pub struct RenderSystem;

impl RenderSystem {
    pub fn update(&mut self, registry: &mut Registry, renderer: &mut VulkanRenderer) -> Result<()> {
        let render_items = registry
            .query2::<Transform, MeshRenderer>()
            .map(|(entity, transform, mesh_renderer)| RenderItem {
                mesh_index: mesh_renderer.mesh,
                transform: transform.clone(),
                alpha: mesh_renderer.material.alpha,
                material_color: mesh_renderer.material.color,
                use_texture: mesh_renderer.material.use_texture,
                texture_index: mesh_renderer.material.texture,
                pipeline_key: mesh_renderer.material.pipeline_key,
                is_visible: registry
                    .get_component::<Visibility>(entity)
                    .is_none_or(|visibility| visibility.is_visible),
            })
            .collect::<Vec<_>>();

        renderer.set_render_items(render_items);

        if let Some(camera_entity) = registry.active_camera_entity() {
            if let Some((_, transform, camera)) = registry
                .query2::<Transform, Camera>()
                .find(|(entity, _, _)| *entity == camera_entity)
            {
                renderer.set_camera(RenderCamera {
                    position: transform.position,
                    target: camera.target,
                    up: vec3(0.0, 0.0, 1.0),
                    fov_y: camera.fov_y,
                    near: camera.near,
                    far: camera.far,
                });
            }
        }

        Ok(())
    }
}
