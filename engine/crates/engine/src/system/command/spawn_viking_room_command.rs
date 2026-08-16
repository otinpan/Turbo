use anyhow::{Result, anyhow};
use cgmath::vec3;
use renderer_vulkan::{PipelineKey, TextureHandle};
use std::collections::HashMap;
use turbo_math::Transform;

use super::{Command, CommandContext};
use crate::{Material, MeshAssetId, MeshRenderer, Resources, Rotator};

#[derive(Clone, Debug)]
pub struct SpawnVikingRoomCommand;

impl Command for SpawnVikingRoomCommand {
    fn id(&self) -> String {
        "spawn_viking_room".to_string()
    }

    fn execute(&self, context: &mut CommandContext<'_>) -> Result<()> {
        let viking_room_mesh3d = use_model(context.resources, "viking_room")?;
        let viking_room_debug_line = use_model(context.resources, "viking_room_debug_line")?;
        let viking_room_lit3d = use_model(context.resources, "viking_room_lit3d")?;
        let viking_texture = use_texture(&context.resources.textures, "viking_room");
        let viking_meshes = [
            viking_room_mesh3d,
            viking_room_debug_line,
            viking_room_lit3d,
        ];
        let index = context
            .world
            .registry
            .entities()
            .iter()
            .filter(|entity| {
                context
                    .world
                    .registry
                    .get_component::<MeshRenderer>(**entity)
                    .is_some_and(|mesh_renderer| {
                        mesh_renderer
                            .asset_id
                            .is_some_and(|asset_id| viking_meshes.contains(&asset_id))
                    })
            })
            .count();

        if context.positions.len() > index {
            let variants = [
                (viking_room_mesh3d, PipelineKey::Mesh3D, 1.0),
                (viking_room_debug_line, PipelineKey::DebugLine3D, 1.0),
                (viking_room_mesh3d, PipelineKey::Transparent3D, 0.5),
                (viking_room_lit3d, PipelineKey::Lit3D, 1.0),
            ];
            let (asset_id, pipeline_key, alpha) = variants[index];
            let mesh = context
                .resources
                .retain_mesh(asset_id)
                .ok_or_else(|| anyhow!("mesh asset not found: {asset_id:?}"))?;
            match MeshRenderer::new(
                mesh,
                Material {
                    color: vec3(1.0, 1.0, 1.0),
                    alpha,
                    use_texture: true,
                    texture: viking_texture,
                    pipeline_key,
                },
            ) {
                Ok(mesh_renderer) => {
                    let mesh_renderer = mesh_renderer.with_asset_id(asset_id);
                    let entity = context.world.spawn();
                    context.world.add_component(
                        entity,
                        Transform {
                            position: context.positions[index],
                            ..Default::default()
                        },
                    );
                    context.world.add_component(entity, mesh_renderer);
                    context.world.add_component(
                        entity,
                        Rotator {
                            speed: vec3(20.0, 0.0, 0.0),
                        },
                    );
                    context.world.set_tags(entity, ["VikingRoom"]);
                }
                Err(e) => {
                    log::error!("Failed to spawn triangle primitive: {e:?}");
                }
            };
        }

        Ok(())
    }
}

fn use_model(resources: &Resources, name: &str) -> Result<MeshAssetId> {
    resources
        .model_asset_id(name)
        .ok_or_else(|| anyhow!("Model not found: {name}"))
}

fn use_texture(textures: &HashMap<String, TextureHandle>, name: &str) -> TextureHandle {
    textures
        .get(name)
        .copied()
        .unwrap_or(crate::app::DEFAULT_TEXTURE)
}
