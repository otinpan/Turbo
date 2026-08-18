use anyhow::Result;
use cgmath::vec3;
use renderer_vulkan::{RenderCamera, RenderItem, VulkanRenderer};
use turbo_math::Transform;

use super::render_command::{RenderCommand, RenderCommandQueue};
use crate::primitive::update_primitive_mesh;
use crate::{Camera, MeshRenderer, Resources, Visibility, World};

pub struct RenderContext<'a> {
    world: &'a mut World,
    resources: &'a mut Resources,
    renderer: &'a mut VulkanRenderer,
    render_commands: &'a mut RenderCommandQueue,
}

impl<'a> RenderContext<'a>{
    pub(crate) fn new(
        world: &'a mut World,
        resources: &'a mut Resources,
        renderer: &'a mut VulkanRenderer,
        render_commands: &'a mut RenderCommandQueue,
    ) -> Self{
        Self { world, resources, renderer, render_commands }
    }
}

#[derive(Clone, Debug)]
pub struct RenderSystem;

impl RenderSystem {
    pub fn update(&mut self, context: &mut RenderContext<'_>) -> Result<()> {
        self.execute_render_commands(context)?;

        let render_items = context
            .world
            .query2::<Transform, MeshRenderer>()
            .map(|(entity, transform, mesh_renderer)| RenderItem {
                mesh_index: mesh_renderer.mesh,
                transform: transform.clone(),
                alpha: mesh_renderer.material.alpha,
                material_color: mesh_renderer.material.color,
                use_texture: mesh_renderer.material.use_texture,
                texture_index: mesh_renderer.material.texture,
                pipeline_key: mesh_renderer.material.pipeline_key,
                is_visible: context
                    .world
                    .get_component::<Visibility>(entity)
                    .is_none_or(|visibility| visibility.is_visible),
            })
            .collect::<Vec<_>>();

        context.renderer.set_render_items(render_items);

        if let Some((_, transform, camera)) = context.world.query2::<Transform, Camera>().next() {
            context.renderer.set_camera(RenderCamera {
                position: transform.position,
                target: camera.target,
                up: vec3(0.0, 0.0, 1.0),
                fov_y: camera.fov_y,
                near: camera.near,
                far: camera.far,
            });
        }

        Ok(())
    }

    pub fn execute_render_commands(&mut self, context: &mut RenderContext<'_>) -> Result<()> {
        let commands = context.render_commands.drain().collect::<Vec<_>>();

        for command in commands {
            match command {
                RenderCommand::DestroyMesh { mesh } => unsafe {
                    context.renderer.destroy_mesh(mesh)?;
                },
                RenderCommand::UpdatePrimitiveMesh {
                    primitive_type,
                    vertex_layout,
                    shape,
                } => {
                    if let Some(mesh) = primitive_mesh(context, primitive_type, vertex_layout) {
                        unsafe {
                            update_primitive_mesh(
                                context.renderer,
                                context.resources,
                                mesh,
                                shape,
                            )?;
                        }
                    }
                }
            }
        }

        Ok(())
    }
}

fn primitive_mesh(
    context: &RenderContext<'_>,
    primitive_type: crate::PrimitiveType,
    vertex_layout: renderer_vulkan::VertexLayout,
) -> Option<crate::PrimitiveMesh> {
    context
        .resources
        .primitive_meshes
        .iter()
        .find(|mesh| mesh.primitive_type == primitive_type && mesh.vertex_layout == vertex_layout)
        .copied()
}
