#[derive(Debug)]
pub enum SceneCommand {
    SetCurrentScene { scene_name: String },
}

#[derive(Debug, Default)]
pub struct SceneCommandQueue {
    commands: Vec<SceneCommand>,
}

impl SceneCommandQueue {
    pub fn set_current_scene(&mut self, scene_name: &str) {
        self.commands.push(SceneCommand::SetCurrentScene {
            scene_name: scene_name.to_string(),
        });
    }

    pub fn drain(&mut self) -> impl Iterator<Item = SceneCommand> + '_ {
        self.commands.drain(..)
    }
}
