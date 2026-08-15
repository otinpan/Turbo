#![allow(
    dead_code,
    unsafe_op_in_unsafe_fn,
    unused_variables,
    clippy::too_many_arguments,
    clippy::unnecessary_wraps
)]

mod camera;
mod material;
mod mesh_renderer;
mod name;
mod rotator;
mod tags;
mod visibility;

pub use camera::Camera;
pub use camera::Camera as CameraComponent;
pub use material::Material;
pub use mesh_renderer::MeshRenderer;
pub use name::Name;
pub use rotator::Rotator;
pub use tags::Tags;
pub use visibility::Visibility;

use turbo_math::Transform;

// Component does not have short reference like &'a str, &String
pub trait Component: 'static {}

impl Component for Transform {}
