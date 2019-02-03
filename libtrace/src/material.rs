use crate::{HitRecord, Ray, Vec3};

pub struct Scatter {
  pub attenuation: Vec3,
  pub scatter: Ray,
}

#[derive(Debug, Clone)]
pub enum Material {
  Lambertian(Lambertian),
  Metal(Metal),
}

impl Material {
  pub fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<Scatter> {
    match self {
      Material::Lambertian(l) => l.scatter(ray, hit_record),
      Material::Metal(m) => m.scatter(ray, hit_record),
    }
  }
}

impl From<Lambertian> for Material {
  fn from(l: Lambertian) -> Material {
    Material::Lambertian(l)
  }
}

impl From<Metal> for Material {
  fn from(m: Metal) -> Material {
    Material::Metal(m)
  }
}

#[derive(Debug, Clone)]
pub struct Lambertian {
  albedo: Vec3,
}

impl Lambertian {
  pub fn new(albedo: Vec3) -> Lambertian {
    Lambertian { albedo }
  }
  fn scatter(&self, _: &Ray, hit_record: &HitRecord) -> Option<Scatter> {
    let target = hit_record.normal + Vec3::random_in_unit_circle();
    let scattered = Ray::new(hit_record.pointing_at, target);
    return Some(Scatter {
      attenuation: self.albedo,
      scatter: scattered,
    });
  }
}

#[derive(Debug, Clone)]
pub struct Metal {
  albedo: Vec3,
  fuzz: f32,
}

impl Metal {
  pub fn new(albedo: Vec3, fuzz: f32) -> Metal {
    let fuzz = if fuzz <= 1. { fuzz } else { 1. };
    Metal { albedo, fuzz }
  }
  fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<Scatter> {
    let reflected = reflect(ray.direction().into_normalized(), hit_record.normal);
    return Some(Scatter {
      scatter: Ray::new(
        hit_record.pointing_at,
        reflected + Vec3::random_in_unit_circle() * self.fuzz,
      ),
      attenuation: self.albedo,
    });
  }
}

fn reflect(vec: Vec3, norm: Vec3) -> Vec3 {
  vec - norm * 2. * vec.dot(&norm)
}
