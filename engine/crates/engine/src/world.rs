use anyhow::Result;
use cgmath::vec3;

pub type Vec3 = cgmath::Vector3<f32>;

#[derive(Clone, Debug)]
pub struct World {
    pub rotate_speed: Vec3,
}

impl World {
    pub fn update(&mut self, delta_time: f32) -> Result<()> {
        Ok(())
    }
}

impl Default for World {
    fn default() -> Self {
        Self {
            rotate_speed: vec3(20.0, 0.0, 0.0),
        }
    }
}
