use crate::sphere::Sphere;
use crate::vec3::Vec3;
use std::fmt::{self, Debug};

#[derive(Clone, Default)]
pub struct Ray {
  origin: Vec3,
  direction: Vec3,
}

impl Debug for Ray {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "Ray({:?} + t * {:?})", self.origin, self.direction)
  }
}

impl Ray {
  pub fn new(origin: Vec3, direction: Vec3) -> Ray {
    Ray { origin, direction }
  }
  pub fn origin(&self) -> &Vec3 {
    &self.origin
  }
  pub fn direction(&self) -> &Vec3 {
    &self.direction
  }

  pub fn point_at(&self, t: f32) -> Vec3 {
    return self.origin.clone() + self.direction.clone().scalar_mult(t);
  }

  pub fn intersects_sphere(&self, sphere: &Sphere) -> Option<f32> {
    let oc = self.origin().clone() - sphere.center().clone();
    let a = self.direction().length();
    let b = 2.0 * oc.clone().dot(self.direction());
    let c = oc.length() - sphere.radius() * sphere.radius();
    let discriminant = b * b - 4. * a * c;
    if discriminant < 0. {
      return None;
    } else {
      return Some((-b - discriminant.sqrt()) / (2.0 * a));
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn test_ray_debug() {
    assert_eq!(
      format!(
        "{:?}",
        Ray::new(Vec3::new(1., 2., 3.), Vec3::new(4., 5., 6.))
      ),
      "Ray(Vec3(1, 2, 3) + t * Vec3(4, 5, 6))".to_string()
    );
  }
}
