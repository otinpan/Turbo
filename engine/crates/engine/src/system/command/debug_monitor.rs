use super::{Command, CommandContext};
use crate::EntityApi;
use anyhow::Result;

#[derive(Clone, Debug)]
pub struct DebugMonitor;

impl Command for DebugMonitor {
    fn id(&self) -> String {
        "debug_monitor".to_string()
    }

    fn execute(&self, context: &mut CommandContext<'_>) -> Result<()> {
        //self.monitor_entities(context)
        self.monitor_mesh_assets(context)
    }
}

impl DebugMonitor {
    fn monitor_entities(&self, context: &mut CommandContext<'_>) -> Result<()> {
        let named_entities = context.get_all_named_entities();
        let taged_entities = context.get_all_taged_entities();

        for (name, entity) in named_entities {
            log::debug!("Named entity: {name} -> {entity:?}");
        }

        for (tag, entity) in taged_entities {
            log::debug!("Tagged entity: {tag} -> {entity:?}");
        }

        Ok(())
    }

    fn monitor_mesh_assets(&self, context: &mut CommandContext<'_>) -> Result<()> {
        for (asset_id, mesh) in context.mesh_assets() {
            log::debug!("Mesh {asset_id:?}: {mesh:?}");
        }

        Ok(())
    }
}
