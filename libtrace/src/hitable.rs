use crate::{Material, Ray, Sphere, Vec3};
use serde_derive::{Deserialize, Serialize};

#[derive(Debug)]
pub struct HitRecord {
  pub t: f32,
  pub pointing_at: Vec3,
  pub normal: Vec3,
  pub material: Material,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Hitable {
  Sphere(Sphere),
  List(Vec<Hitable>),
}

impl From<Sphere> for Hitable {
  #[inline]
  fn from(sphere: Sphere) -> Hitable {
    Hitable::Sphere(sphere)
  }
}

impl From<Vec<Hitable>> for Hitable {
  #[inline]
  fn from(list: Vec<Hitable>) -> Hitable {
    Hitable::List(list)
  }
}

impl Hitable {
  pub fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
    match self {
      Hitable::Sphere(s) => s.hit(ray, t_min, t_max),
      Hitable::List(list) => {
        let mut closest_so_far = t_max;
        let mut result: Option<HitRecord> = None;
        for hitable in list.iter() {
          if let Some(hit_record) = hitable.hit(ray, t_min, closest_so_far) {
            closest_so_far = hit_record.t;
            result = Some(hit_record);
          }
        }
        return result;
      }
    }
  }
}
