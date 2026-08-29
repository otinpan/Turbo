use std::collections::HashMap;

use anyhow::{Result, anyhow};

use crate::{Scene, SceneContext, SceneId, UpdateContext};

pub struct RegisteredScene {
    scene_id: SceneId,
    scene: Box<dyn Scene>,
}

impl RegisteredScene {
    pub fn scene_id(&self) -> SceneId {
        self.scene_id
    }
}

pub struct SceneManager {
    next_scene_id: usize,
    scenes: HashMap<String, RegisteredScene>,
    current_scene: Option<String>,
    previous_scene: Option<String>,
    next_scene: Option<String>,
}

impl SceneManager {
    pub fn add_scene<S>(&mut self, scene: S) -> Result<SceneId>
    where
        S: Scene + 'static,
    {
        self.add_boxed_scene(Box::new(scene))
    }

    pub fn add_boxed_scene(&mut self, scene: Box<dyn Scene>) -> Result<SceneId> {
        let name = scene.name();

        if self.is_registered_scene(&name) {
            return Err(anyhow!("scene already registered: {name}"));
        }

        let scene_id = SceneId(self.next_scene_id);
        self.next_scene_id += 1;

        self.scenes
            .insert(name, RegisteredScene { scene_id, scene });

        Ok(scene_id)
    }

    pub fn is_registered_scene(&self, name: &str) -> bool {
        self.scenes.contains_key(name)
    }

    pub fn scene_id(&self, name: &str) -> Option<SceneId> {
        self.scenes.get(name).map(|registered| registered.scene_id)
    }

    pub fn current_scene_id(&self) -> Option<SceneId> {
        self.current_scene
            .as_deref()
            .and_then(|name| self.scene_id(name))
    }

    pub fn get_scene(&self, name: &str) -> Option<&(dyn Scene + '_)> {
        self.scenes
            .get(name)
            .map(|registered| registered.scene.as_ref())
    }

    pub fn get_scene_mut(&mut self, name: &str) -> Option<&mut (dyn Scene + '_)> {
        match self.scenes.get_mut(name) {
            Some(registered) => Some(registered.scene.as_mut()),
            None => None,
        }
    }

    pub fn enter_current_scene(&mut self, context: &mut SceneContext<'_>) -> Result<()> {
        let name = self
            .current_scene
            .clone()
            .ok_or_else(|| anyhow!("current scene is not set"))?;

        let registered = self
            .scenes
            .get_mut(&name)
            .ok_or_else(|| anyhow!("scene not registered: {name}"))?;

        registered.scene.on_enter(context)
    }

    pub fn update_current_scene(&mut self, context: &mut UpdateContext<'_>) -> Result<()> {
        let Some(name) = self.current_scene.clone() else {
            return Ok(());
        };

        let registered = self
            .scenes
            .get_mut(&name)
            .ok_or_else(|| anyhow!("scene not registered: {name}"))?;

        registered.scene.update(context)
    }

    pub fn exit_current_scene(&mut self, context: &mut SceneContext<'_>) -> Result<()> {
        let name = self
            .current_scene
            .clone()
            .ok_or_else(|| anyhow!("current scene is not set"))?;

        let registered = self
            .scenes
            .get_mut(&name)
            .ok_or_else(|| anyhow!("scene not registered: {name}"))?;

        registered.scene.on_exit(context)
    }

    pub fn current_scene_name(&self) -> Option<&str> {
        self.current_scene.as_deref()
    }

    pub fn previous_scene_name(&self) -> Option<&str> {
        self.previous_scene.as_deref()
    }

    pub fn next_scene_name(&self) -> Option<&str> {
        self.next_scene.as_deref()
    }

    pub fn set_current_scene(&mut self, name: &str) -> Result<SceneId> {
        let scene_id = self
            .scene_id(name)
            .ok_or_else(|| anyhow!("scene not registered: {name}"))?;

        self.previous_scene = self.current_scene.clone();
        self.current_scene = Some(name.to_string());
        Ok(scene_id)
    }

    pub fn change_scene(&mut self, name: &str) -> Result<SceneId> {
        let scene_id = self
            .scene_id(name)
            .ok_or_else(|| anyhow!("scene not registered: {name}"))?;

        self.next_scene = Some(name.to_string());
        Ok(scene_id)
    }

    fn take_next_scene(&mut self) -> Option<String> {
        self.next_scene.take()
    }

    pub fn set_previous_scene(&mut self, name: Option<String>) {
        self.previous_scene = name;
    }
}

impl Default for SceneManager {
    fn default() -> Self {
        Self {
            next_scene_id: 0,
            scenes: HashMap::new(),
            current_scene: None,
            previous_scene: None,
            next_scene: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{SceneContext, UpdateContext};

    struct TestScene {
        name: &'static str,
    }

    impl Scene for TestScene {
        fn name(&self) -> String {
            self.name.to_string()
        }

        fn on_enter(&mut self, _context: &mut SceneContext<'_>) -> Result<()> {
            Ok(())
        }

        fn update(&mut self, _context: &mut UpdateContext<'_>) -> Result<()> {
            Ok(())
        }

        fn on_exit(&mut self, _context: &mut SceneContext<'_>) -> Result<()> {
            Ok(())
        }
    }

    #[test]
    fn registers_scene_and_assigns_scene_id() {
        let mut scene_manager = SceneManager::default();

        let scene_id = scene_manager
            .add_scene(TestScene { name: "basic_3d" })
            .unwrap();

        assert_eq!(scene_id, SceneId(0));
        assert!(scene_manager.is_registered_scene("basic_3d"));
        assert_eq!(scene_manager.scene_id("basic_3d"), Some(SceneId(0)));
        assert_eq!(
            scene_manager.get_scene("basic_3d").unwrap().name(),
            "basic_3d"
        );
        assert!(scene_manager.get_scene("missing").is_none());
    }

    #[test]
    fn add_scene_rejects_duplicate_scene_name() {
        let mut scene_manager = SceneManager::default();

        assert!(
            scene_manager
                .add_scene(TestScene { name: "basic_3d" })
                .is_ok()
        );
        assert!(
            scene_manager
                .add_scene(TestScene { name: "basic_3d" })
                .is_err()
        );
        assert_eq!(scene_manager.scene_id("basic_3d"), Some(SceneId(0)));
    }

    #[test]
    fn add_scene_increments_scene_id_for_each_registered_scene() {
        let mut scene_manager = SceneManager::default();

        assert_eq!(
            scene_manager
                .add_scene(TestScene { name: "first" })
                .unwrap(),
            SceneId(0)
        );
        assert_eq!(
            scene_manager
                .add_scene(TestScene { name: "second" })
                .unwrap(),
            SceneId(1)
        );
        assert_eq!(
            scene_manager
                .add_scene(TestScene { name: "third" })
                .unwrap(),
            SceneId(2)
        );
    }

    #[test]
    fn current_and_next_scene_accept_only_registered_scenes() {
        let mut scene_manager = SceneManager::default();
        scene_manager
            .add_scene(TestScene { name: "first" })
            .unwrap();
        scene_manager
            .add_scene(TestScene { name: "second" })
            .unwrap();

        assert!(scene_manager.set_current_scene("missing").is_err());
        assert_eq!(scene_manager.current_scene_name(), None);

        assert_eq!(
            scene_manager.set_current_scene("first").unwrap(),
            SceneId(0)
        );
        assert_eq!(scene_manager.current_scene_name(), Some("first"));
        assert_eq!(scene_manager.current_scene_id(), Some(SceneId(0)));

        assert!(scene_manager.change_scene("missing").is_err());
        assert_eq!(scene_manager.next_scene_name(), None);

        assert_eq!(scene_manager.change_scene("second").unwrap(), SceneId(1));
        assert_eq!(scene_manager.next_scene_name(), Some("second"));
        assert_eq!(scene_manager.take_next_scene(), Some("second".to_string()));
        assert_eq!(scene_manager.next_scene_name(), None);
    }
}
