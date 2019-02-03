use crate::{HitRecord, Hitable, Material, Ray, Vec3};

#[derive(Debug)]
pub struct Sphere {
  center: Vec3,
  radius: f32,
  material: Material,
}

impl Sphere {
  pub fn new(radius: f32, center: Vec3, material: Material) -> Sphere {
    Sphere {
      radius,
      center,
      material,
    }
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
      let pointing_at = ray.point_at(temp);
      let normal = (pointing_at - *self.center()).scalar_div(self.radius);
      let result = Some(HitRecord {
        t: temp,
        pointing_at,
        normal,
        material: self.material.clone(),
      });
      return result;
    } else {
      let temp = (-b + discriminant) / a;
      if temp < t_max && temp > t_min {
        let pointing_at = ray.point_at(temp);
        return Some(HitRecord {
          t: temp,
          pointing_at,
          normal: (pointing_at - *self.center()).scalar_div(self.radius),
          material: self.material.clone(),
        });
      }
    }
    None
  }
}
