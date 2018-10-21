use crate::vec3::Vec3;

#[derive(Debug, PartialEq, Clone)]
pub struct Sphere {
  center: Vec3,
  radius: f32,
}

impl Default for Sphere {
  fn default() -> Sphere {
    Sphere {
      radius: 1.,
      center: Vec3::default(),
    }
  }
}

impl Sphere {
  pub fn new(radius: f32, center: Vec3) -> Sphere {
    Sphere { radius, center }
  }

  pub fn radius(&self) -> f32 {
    self.radius
  }

  pub fn center(&self) -> &Vec3 {
    &self.center
  }
}
