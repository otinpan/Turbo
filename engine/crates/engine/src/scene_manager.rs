use std::collections::HashMap;

use crate::Scene;

pub struct SceneManager {
    scenes: HashMap<String, Box<dyn Scene>>,
    current_scene: Option<String>,
    previous_scene: Option<String>,
    next_scene: Option<String>,
}

impl SceneManager {
    pub fn add_scene<S>(&mut self, scene: S)
    where
        S: Scene + 'static,
    {
        let name = scene.name();
        self.scenes.insert(name, Box::new(scene));
    }

    pub fn add_boxed_scene(&mut self, scene: Box<dyn Scene>) {
        let name = scene.name();
        self.scenes.insert(name, scene);
    }

    pub fn is_registered_scene(&self, name: &str) -> bool {
        self.scenes.contains_key(name)
    }

    pub fn get_scene(&self, name: &str) -> Option<&(dyn Scene + '_)> {
        self.scenes.get(name).map(|scene| scene.as_ref())
    }

    pub fn get_scene_mut(&mut self, name: &str) -> Option<&mut (dyn Scene + '_)> {
        match self.scenes.get_mut(name) {
            Some(scene) => Some(scene.as_mut()),
            None => None,
        }
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

    pub fn set_current_scene(&mut self, name: &str) -> bool {
        if !self.is_registered_scene(name) {
            return false;
        }

        self.current_scene = Some(name.to_string());
        true
    }

    pub fn change_scene(&mut self, name: &str) -> bool {
        if !self.is_registered_scene(name) {
            return false;
        }

        self.next_scene = Some(name.to_string());
        true
    }

    pub fn take_next_scene(&mut self) -> Option<String> {
        self.next_scene.take()
    }

    pub fn set_previous_scene(&mut self, name: Option<String>) {
        self.previous_scene = name;
    }
}

impl Default for SceneManager {
    fn default() -> Self {
        Self {
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
    use anyhow::Result;

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
    fn registers_and_gets_scene() {
        let mut scene_manager = SceneManager::default();

        scene_manager.add_scene(TestScene { name: "basic_3d" });

        assert!(scene_manager.is_registered_scene("basic_3d"));
        assert!(!scene_manager.is_registered_scene("missing"));
        assert_eq!(
            scene_manager.get_scene("basic_3d").unwrap().name(),
            "basic_3d"
        );
        assert!(scene_manager.get_scene("missing").is_none());
    }

    #[test]
    fn current_and_next_scene_accept_only_registered_scenes() {
        let mut scene_manager = SceneManager::default();
        scene_manager.add_scene(TestScene { name: "first" });
        scene_manager.add_scene(TestScene { name: "second" });

        assert!(!scene_manager.set_current_scene("missing"));
        assert_eq!(scene_manager.current_scene_name(), None);

        assert!(scene_manager.set_current_scene("first"));
        assert_eq!(scene_manager.current_scene_name(), Some("first"));

        assert!(!scene_manager.change_scene("missing"));
        assert_eq!(scene_manager.next_scene_name(), None);

        assert!(scene_manager.change_scene("second"));
        assert_eq!(scene_manager.next_scene_name(), Some("second"));
        assert_eq!(scene_manager.take_next_scene(), Some("second".to_string()));
        assert_eq!(scene_manager.next_scene_name(), None);
    }
}
