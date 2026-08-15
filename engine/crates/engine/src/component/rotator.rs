use super::Component;
use cgmath::Vector3;

#[derive(Clone, Debug)]
pub struct Rotator {
    pub speed: Vector3<f32>,
}

impl Component for Rotator {}
