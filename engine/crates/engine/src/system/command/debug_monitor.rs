use super::{Command, CommandContext};
use anyhow::Result;

#[derive(Clone, Debug)]
pub struct DebugMonitor;

impl Command for DebugMonitor {
    fn id(&self) -> String {
        "debug_monitor".to_string()
    }

    fn execute(&self, context: &mut CommandContext<'_>) -> Result<()> {
        let named_entities = context.world.get_all_named_entities();
        let taged_entities = context.world.get_all_taged_entities();

        for (name, entity) in named_entities {
            log::debug!("Named entity: {name} -> {entity:?}");
        }

        for (tag, entity) in taged_entities {
            log::debug!("Tagged entity: {tag} -> {entity:?}");
        }

        Ok(())
    }
}
