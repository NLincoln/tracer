use crate::{Ray, Vec3};

#[derive(Debug, Clone, PartialEq)]
pub struct HitRecord {
  pub t: f32,
  pub p: Vec3,
  pub normal: Vec3,
}

pub trait Hitable {
  fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
}

impl Hitable for Vec<Box<dyn Hitable + Sync>> {
  fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
    return (&self[..]).hit(ray, t_min, t_max);
  }
}

impl<'a> Hitable for &'a [Box<dyn Hitable + Sync>] {
  fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
    let mut closest_so_far = t_max;
    let mut result: Option<HitRecord> = None;
    for hitable in self.iter() {
      if let Some(hit_record) = hitable.hit(ray, t_min, closest_so_far) {
        closest_so_far = hit_record.t;
        result = Some(hit_record);
      }
    }
    return result;
  }
}
