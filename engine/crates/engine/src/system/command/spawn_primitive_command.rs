use anyhow::Result;
use cgmath::vec3;
use renderer_vulkan::{MeshHandle, PipelineKey, TextureHandle};
use std::collections::HashMap;
use turbo_math::Transform;

use super::{Command, CommandContext};
use crate::Material;
use crate::app::DEFAULT_TEXTURE;
use crate::primitive::{PrimitiveMesh, PrimitiveType, spawn_primitive_from_mesh};

#[derive(Clone, Debug)]
pub struct SpawnPrimitiveCommand {
    pub primitive_type: PrimitiveType,
    pub pipeline_key: PipelineKey,
    pub texture_name: Option<&'static str>,
}

impl Command for SpawnPrimitiveCommand {
    fn id(&self) -> String {
        format!(
            "spawn_primitive:{:?}:{:?}:{:?}",
            self.primitive_type, self.pipeline_key, self.texture_name
        )
    }

    fn execute(&self, context: &mut CommandContext<'_>) -> Result<()> {
        let position = mouse_position_on_spawn_plane(context);
        let texture = self
            .texture_name
            .map(|name| use_texture(&context.resources.textures, name))
            .unwrap_or(DEFAULT_TEXTURE);

        if let Some(mesh) =
            primitive_handle(&context.resources.primitive_meshes, self.primitive_type)
        {
            if let Err(e) = spawn_primitive_from_mesh(
                context.world,
                mesh,
                Material {
                    color: vec3(1.0, 1.0, 1.0),
                    use_texture: true,
                    texture,
                    pipeline_key: self.pipeline_key,
                    ..Default::default()
                },
                Transform {
                    position,
                    ..Default::default()
                },
            ) {
                log::error!("Failed to spawn {:?} primitive: {e:?}", self.primitive_type);
            }
        }

        Ok(())
    }
}

fn primitive_handle(
    primitive_meshes: &[PrimitiveMesh],
    primitive_type: PrimitiveType,
) -> Option<MeshHandle> {
    primitive_meshes
        .iter()
        .find(|mesh| mesh.primitive_type == primitive_type)
        .map(|mesh| mesh.handle)
}

fn mouse_position_on_spawn_plane(context: &CommandContext<'_>) -> cgmath::Vector3<f32> {
    let mouse = context.input.mouse_position();
    let window_size = context.input.window_size();
    let width = window_size.x.max(1.0);
    let height = window_size.y.max(1.0);
    let aspect = width / height;
    let world_height = 4.0;

    let x = mouse.x / width - 0.5;
    let y = 0.5 - mouse.y / height;

    vec3(0.0, x * world_height * aspect, y * world_height)
}

fn use_texture(textures: &HashMap<String, TextureHandle>, name: &str) -> TextureHandle {
    textures.get(name).copied().unwrap_or(DEFAULT_TEXTURE)
}
