use anyhow::Result;
use cgmath::vec3;
use renderer_vulkan::PipelineKey;
use turbo_math::Transform;

use super::{Command, CommandContext};
use crate::{Material, MeshRenderer, Rotator};

#[derive(Clone, Debug)]
pub struct SpawnVikingRoomCommand;

impl Command for SpawnVikingRoomCommand {
    fn id(&self) -> String {
        "spawn_viking_room".to_string()
    }

    fn execute(&self, context: &mut CommandContext<'_>) -> Result<()> {
        let viking_room_mesh3d = context.model_asset_id("viking_room")?;
        let viking_room_debug_line = context.model_asset_id("viking_room_debug_line")?;
        let viking_room_lit3d = context.model_asset_id("viking_room_lit3d")?;
        let viking_texture = context.texture("viking_room")?;
        let viking_meshes = [
            viking_room_mesh3d,
            viking_room_debug_line,
            viking_room_lit3d,
        ];
        let entities = context.entities().to_vec();
        let index = entities
            .iter()
            .filter(|entity| {
                context
                    .get_component::<MeshRenderer>(**entity)
                    .is_some_and(|mesh_renderer| {
                        mesh_renderer
                            .asset_id
                            .is_some_and(|asset_id| viking_meshes.contains(&asset_id))
                    })
            })
            .count();

        if context.positions().len() > index {
            let variants = [
                ("viking_room", PipelineKey::Mesh3D, 1.0),
                ("viking_room_debug_line", PipelineKey::DebugLine3D, 1.0),
                ("viking_room", PipelineKey::Transparent3D, 0.5),
                ("viking_room_lit3d", PipelineKey::Lit3D, 1.0),
            ];

            let entity = context.spawn_model(
                variants[index].0,
                Transform {
                    position: context.positions()[index],
                    ..Default::default()
                },
                Material {
                    color: vec3(1.0, 1.0, 1.0),
                    alpha: variants[index].2,
                    use_texture: true,
                    texture: viking_texture,
                    pipeline_key: variants[index].1,
                },
            )?;
            context.add_component(
                entity,
                Rotator {
                    speed: vec3(20.0, 0.0, 0.0),
                },
            );
            context.set_tags(entity, ["Model", "VikingRoom", variants[index].0]);
        }

        Ok(())
    }
}
