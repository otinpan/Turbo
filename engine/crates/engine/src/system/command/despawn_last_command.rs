use anyhow::Result;

use super::{Command, CommandContext};

#[derive(Clone, Debug)]
pub struct DespawnLastCommand;

impl Command for DespawnLastCommand {
    fn id(&self) -> String {
        "despawn_last".to_string()
    }

    fn execute(&self, context: &mut CommandContext<'_>) -> Result<()> {
        let id = context.world.entities().last().copied();

        if let Some(id) = id {
            let asset_id = context
                .world
                .get_component::<crate::MeshRenderer>(id)
                .and_then(|mesh_renderer| mesh_renderer.asset_id);

            if let Some(asset_id) = asset_id {
                if let Some(mesh) = context.resources.release_mesh(asset_id) {
                    context.render_commands.destroy_mesh(mesh);
                }
            }
            context.world.despawn(id);
        }

        Ok(())
    }
}
