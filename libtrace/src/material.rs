use crate::texture::Texture;
use crate::{HitRecord, Ray, Vec3};
use serde_derive::{Deserialize, Serialize};

pub struct Scatter {
    pub attenuation: Vec3,
    pub scatter: Ray,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "type")]
pub enum Material {
    Lambertian(Lambertian),
    Metal(Metal),
    Dialectric(Dialectric),
    Diffuse(Diffuse),
}

impl Material {
    #[inline]
    pub fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<Scatter> {
        match self {
            Material::Lambertian(l) => l.scatter(ray, hit_record),
            Material::Metal(m) => m.scatter(ray, hit_record),
            Material::Dialectric(d) => d.scatter(ray, hit_record),
            Material::Diffuse(diff) => diff.scatter(ray, hit_record),
        }
    }

    pub fn emitted(&self, uv: (f32, f32), p: Vec3) -> Vec3 {
        match self {
            Material::Diffuse(d) => d.emitted(uv, p),
            Material::Lambertian(_) | Material::Metal(_) | Material::Dialectric(_) => 0f32.into(),
        }
    }
}

impl From<Lambertian> for Material {
    #[inline]
    fn from(l: Lambertian) -> Material {
        Material::Lambertian(l)
    }
}

impl From<Metal> for Material {
    #[inline]
    fn from(m: Metal) -> Material {
        Material::Metal(m)
    }
}
impl From<Dialectric> for Material {
    #[inline]
    fn from(d: Dialectric) -> Material {
        Material::Dialectric(d)
    }
}

impl From<Diffuse> for Material {
    fn from(d: Diffuse) -> Material {
        Material::Diffuse(d)
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Lambertian {
    albedo: Texture,
}

impl Lambertian {
    #[inline]
    pub fn new<T: Into<Texture>>(albedo: T) -> Lambertian {
        Lambertian {
            albedo: albedo.into(),
        }
    }
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<Scatter> {
        let target = hit_record.normal + Vec3::random_in_unit_circle();
        let scattered = Ray::new(hit_record.pointing_at, target, ray.time());
        Some(Scatter {
            attenuation: self.albedo.value(
                hit_record.uv.0,
                hit_record.uv.1,
                hit_record.pointing_at,
            ),
            scatter: scattered,
        })
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Metal {
    albedo: Texture,
    fuzz: f32,
}

impl Metal {
    pub fn new<T: Into<Texture>>(albedo: T, fuzz: f32) -> Metal {
        let fuzz = if fuzz <= 1. { fuzz } else { 1. };
        Metal {
            albedo: albedo.into(),
            fuzz,
        }
    }
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<Scatter> {
        let reflected = reflect(ray.direction().into_normalized(), hit_record.normal);
        Some(Scatter {
            scatter: Ray::new(
                hit_record.pointing_at,
                reflected + Vec3::random_in_unit_circle() * self.fuzz,
                ray.time(),
            ),
            attenuation: self.albedo.value(
                hit_record.uv.0,
                hit_record.uv.1,
                hit_record.pointing_at,
            ),
        })
    }
}

#[inline]
fn reflect(vec: Vec3, norm: Vec3) -> Vec3 {
    vec - norm * 2. * vec.dot(norm)
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Dialectric {
    ref_idx: f32,
}

impl Dialectric {
    pub fn new(ref_idx: f32) -> Dialectric {
        Dialectric { ref_idx }
    }
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<Scatter> {
        let (outward_normal, ni_over_nt, cosine) = if ray.direction().dot(hit_record.normal) > 0. {
            let cosine =
                self.ref_idx * ray.direction().dot(hit_record.normal) / ray.direction().length();

            (-hit_record.normal, self.ref_idx, cosine)
        } else {
            let cosine = -ray.direction().dot(hit_record.normal) / ray.direction().length();
            (hit_record.normal, 1.0 / self.ref_idx, cosine)
        };

        let (reflect_prob, refracted) = match refract(&ray.direction(), &outward_normal, ni_over_nt)
        {
            Some(refracted) => (schlick(cosine, self.ref_idx), Some(refracted)),
            None => (1.0, None),
        };
        let reflected = reflect(ray.direction(), hit_record.normal);
        let scatter = match refracted {
            Some(refracted) => {
                if rand::random::<f32>() < reflect_prob {
                    Ray::new(hit_record.pointing_at, reflected, ray.time())
                } else {
                    Ray::new(hit_record.pointing_at, refracted, ray.time())
                }
            }
            None => Ray::new(hit_record.pointing_at, reflected, ray.time()),
        };

        Some(Scatter {
            scatter,
            attenuation: Vec3::new(1., 1., 1.),
        })
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Diffuse {
    emit: Texture,
}

impl Diffuse {
    pub fn new<T: Into<Texture>>(emit: T) -> Diffuse {
        Diffuse { emit: emit.into() }
    }
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<Scatter> {
        None
    }
    fn emitted(&self, (u, v): (f32, f32), p: Vec3) -> Vec3 {
        self.emit.value(u, v, p)
    }
}

#[inline]
fn schlick(cosine: f32, ref_idx: f32) -> f32 {
    let mut r0 = (1. - ref_idx) / (1. + ref_idx);
    r0 *= 2.;
    r0 + (1. - r0) * (1. - cosine).powi(5)
}
#[inline]
fn refract(vec: &Vec3, norm: &Vec3, ni_over_nt: f32) -> Option<Vec3> {
    let unit_vector = vec.into_normalized();
    let dt = unit_vector.dot(*norm);
    let discriminant = 1.0 - ni_over_nt * ni_over_nt * (1. - dt * dt);
    if discriminant > 0. {
        Some((unit_vector - *norm * dt) * ni_over_nt - *norm * discriminant.sqrt())
    } else {
        None
    }
}
