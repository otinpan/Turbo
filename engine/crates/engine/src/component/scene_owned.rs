use crate::Component;
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SceneId(pub usize);

#[derive(Clone, Debug)]
pub struct SceneOwned {
    pub scene_id: SceneId,
}

impl Component for SceneOwned {}
