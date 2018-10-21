use crate::{Ray, Vec3};

#[derive(Debug, PartialEq, Clone)]
pub struct Camera {
  origin: Vec3,
  lower_left_corner: Vec3,
  horizontal: Vec3,
  vertical: Vec3,
}

impl Camera {
  pub fn new() -> Camera {
    Camera {
      origin: (0., 0., 0.).into(),
      lower_left_corner: (-2., -1., -1.).into(),
      horizontal: (4., 0., 0.).into(),
      vertical: (0., 2., 0.).into(),
    }
  }

  pub fn get_ray(&self, u: f32, v: f32) -> Ray {
    Ray::new(
      self.origin,
      self.lower_left_corner + self.horizontal * u + self.vertical * v,
    )
  }
}
