use anyhow::Result;
use cgmath::vec3;
use turbo_math::Transform;
pub type Vec3 = cgmath::Vector3<f32>;

#[derive(Clone, Debug)]
pub struct World {
    pub rotate_speed: Vec3,
    objects: Vec<WorldObject>,
}

impl World {
    pub fn spawn(&mut self, mesh: MeshHandle, transform: Transform) {
        self.objects.push(WorldObject { transform, mesh });
    }

    pub fn objects(&self) -> &[WorldObject] {
        &self.objects
    }

    pub fn update(&mut self, delta_time: f32) -> Result<()> {
        for object in &mut self.objects {
            object.transform.rotate(self.rotate_speed * delta_time);
        }

        Ok(())
    }
}

impl Default for World {
    fn default() -> Self {
        Self {
            rotate_speed: vec3(20.0, 0.0, 0.0),
            objects: Vec::new(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct WorldObject {
    pub transform: Transform,
    pub mesh: MeshHandle,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct MeshHandle(pub usize);
