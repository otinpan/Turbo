use crate::SceneCommandQueue;

pub trait SceneCommandApi {
    fn scene_commands_mut(&mut self) -> &mut SceneCommandQueue;

    fn set_current_scene(&mut self, scene_name: &str) {
        self.scene_commands_mut().set_current_scene(scene_name);
    }
}
