use anyhow::Result;

use super::{Command, CommandContext};

#[derive(Clone, Debug)]
pub struct DespawnLastCommand;

impl Command for DespawnLastCommand {
    fn id(&self) -> String {
        "despawn_last".to_string()
    }

    fn execute(&self, context: &mut CommandContext<'_>) -> Result<()> {
        let id = context.world.registry.entities().last().copied();

        if let Some(id) = id {
            if let Some(mesh_renderer) = context.world.get_component::<crate::MeshRenderer>(id) {
                if let Some(asset_id) = mesh_renderer.asset_id {
                    context.resources.release_mesh_for_renderer(asset_id);
                }
            }
            context.world.despawn(id);
        }

        Ok(())
    }
}
