// Camera ///////////////////////////////////////
#[derive(Clone, Debug)]
pub struct CameraComponent {
    pub fov_y: f32,
    pub near: f32,
    pub far: f32,
}