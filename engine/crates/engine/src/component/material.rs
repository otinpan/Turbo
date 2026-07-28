use cgmath::Vector3;

pub type Vec3=Vector3<f32>;

#[derive(Copy,Clone,Debug,PartialEq)]
pub struct Material{
  pub color: Vec3,
}

impl Material{
  pub const fn new(color: Vec3) -> Self{
    Self{color}
  }
}

impl Default for Material{
  fn default() -> Self{
    Self { 
      color: cgmath::vec3(1.0,1.0,1.0)
    }
  }
}