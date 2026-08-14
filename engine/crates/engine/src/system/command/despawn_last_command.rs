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
            context.world.despawn(id);
        }

        Ok(())
    }
}
