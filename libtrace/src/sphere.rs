use crate::{HitRecord, Hitable, Ray, Vec3};

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

impl Hitable for Sphere {
  fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
    let oc = *ray.origin() - *self.center();
    let a = ray.direction().clone().dot(ray.direction());
    let b = oc.dot(ray.direction());
    let c = oc.squared_length() - self.radius() * self.radius();
    let discriminant = b * b - a * c;
    if discriminant < 0. {
      return None;
    }
    let discriminant = discriminant.sqrt();
    let temp = (-b - discriminant) / a;
    if temp < t_max && temp > t_min {
      let p = ray.point_at(temp);
      let normal = (p - *self.center()).scalar_div(self.radius);
      let result = Some(HitRecord {
        t: temp,
        p: p.clone(),
        normal,
      });
      return result;
    } else {
      let temp = (-b + discriminant) / a;
      if temp < t_max && temp > t_min {
        let p = ray.point_at(temp);
        return Some(HitRecord {
          t: temp,
          p: p.clone(),
          normal: (p - *self.center()).scalar_div(self.radius),
        });
      }
    }
    None
  }
}
