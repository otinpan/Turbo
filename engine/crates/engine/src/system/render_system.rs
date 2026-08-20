use anyhow::Result;
use cgmath::vec3;
use renderer_vulkan::{RenderCamera, RenderItem, VertexLayout, VulkanRenderer};
use turbo_math::Transform;

use super::render_command::{RenderCommand, RenderCommandQueue};
use crate::primitive::{PrimitiveMesh, PrimitiveType, update_primitive_mesh};
use crate::{Camera, Component, EntityId, MeshAssetId, MeshRenderer, Resources, Visibility, World};

pub struct RenderContext<'a> {
    world: &'a mut World,
    resources: &'a mut Resources,
    renderer: &'a mut VulkanRenderer,
    render_commands: &'a mut RenderCommandQueue,
}

impl<'a> RenderContext<'a> {
    pub(crate) fn new(
        world: &'a mut World,
        resources: &'a mut Resources,
        renderer: &'a mut VulkanRenderer,
        render_commands: &'a mut RenderCommandQueue,
    ) -> Self {
        Self {
            world,
            resources,
            renderer,
            render_commands,
        }
    }

    pub fn query2<A, B>(&self) -> Box<dyn Iterator<Item = (EntityId, &A, &B)> + '_>
    where
        A: Component,
        B: Component,
    {
        self.world.query2::<A, B>()
    }

    pub fn get_component<T: Component>(&self, entity: EntityId) -> Option<&T> {
        self.world.get_component::<T>(entity)
    }

    pub fn primitive_asset_id(
        &self,
        primitive_type: PrimitiveType,
        vertex_layout: VertexLayout,
    ) -> Option<MeshAssetId> {
        self.resources
            .primitive_asset_id(primitive_type, vertex_layout)
    }

    pub(crate) fn set_render_items(&mut self, render_items: Vec<RenderItem>) {
        self.renderer.set_render_items(render_items);
    }

    pub(crate) fn set_camera(&mut self, camera: RenderCamera) {
        self.renderer.set_camera(camera);
    }

    pub(crate) unsafe fn destroy_mesh(&mut self, mesh: renderer_vulkan::MeshHandle) -> Result<()> {
        self.renderer.destroy_mesh(mesh)
    }

    pub(crate) unsafe fn update_primitive_mesh(
        &mut self,
        mesh: PrimitiveMesh,
        shape: crate::PrimitiveShape,
    ) -> Result<()> {
        update_primitive_mesh(self.renderer, self.resources, mesh, shape)
    }

    pub(crate) fn drain_render_commands(&mut self) -> Vec<RenderCommand> {
        self.render_commands.drain().collect()
    }
}

#[derive(Clone, Debug)]
pub struct RenderSystem;

impl RenderSystem {
    pub fn update(&mut self, context: &mut RenderContext<'_>) -> Result<()> {
        self.execute_render_commands(context)?;

        let render_items = context
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
                    .get_component::<Visibility>(entity)
                    .is_none_or(|visibility| visibility.is_visible),
            })
            .collect::<Vec<_>>();

        context.set_render_items(render_items);

        let render_camera =
            context
                .query2::<Transform, Camera>()
                .next()
                .map(|(_, transform, camera)| RenderCamera {
                    position: transform.position,
                    target: camera.target,
                    up: vec3(0.0, 0.0, 1.0),
                    fov_y: camera.fov_y,
                    near: camera.near,
                    far: camera.far,
                });

        if let Some(render_camera) = render_camera {
            context.set_camera(render_camera);
        }

        Ok(())
    }

    pub fn execute_render_commands(&mut self, context: &mut RenderContext<'_>) -> Result<()> {
        let commands = context.drain_render_commands();

        for command in commands {
            match command {
                RenderCommand::DestroyMesh { mesh } => unsafe {
                    context.destroy_mesh(mesh)?;
                },
                RenderCommand::UpdatePrimitiveMesh {
                    primitive_type,
                    vertex_layout,
                    shape,
                } => {
                    if let Some(asset_id) =
                        context.primitive_asset_id(primitive_type, vertex_layout)
                    {
                        let primitive_mesh = PrimitiveMesh {
                            asset_id,
                            primitive_type,
                            vertex_layout,
                        };
                        unsafe {
                            context.update_primitive_mesh(primitive_mesh, shape)?;
                        }
                    }
                }
            }
        }

        Ok(())
    }
}
