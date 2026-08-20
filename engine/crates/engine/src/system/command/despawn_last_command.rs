use anyhow::Result;

use super::{Command, CommandContext};

#[derive(Clone, Debug)]
pub struct DespawnLastCommand;

impl Command for DespawnLastCommand {
    fn id(&self) -> String {
        "despawn_last".to_string()
    }

    fn execute(&self, context: &mut CommandContext<'_>) -> Result<()> {
        context.despawn_last();

        Ok(())
    }
}
