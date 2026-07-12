// Camera ///////////////////////////////////////
#[derive(Clone, Debug)]
pub struct CameraComponent {
    pub target: cgmath::Vector3<f32>,
    pub fov_y: f32,
    pub near: f32,
    pub far: f32,
    pub yaw: f32,
    pub pitch: f32,
}
