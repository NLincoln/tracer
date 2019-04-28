use crate::{HitRecord, Material, Ray, Vec3};
use serde_derive::{Deserialize, Serialize};

pub trait Sphere {
    fn center(&self, time: f32) -> Vec3;
    fn radius(&self) -> f32;
    fn material(&self) -> Material;

    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let center = self.center(ray.time());
        let radius = self.radius();

        let oc = *ray.origin() - center;
        let a = ray.direction().clone().dot(ray.direction());
        let b = oc.dot(ray.direction());
        let c = oc.squared_length() - radius * radius;
        let discriminant = b * b - a * c;
        if discriminant < 0. {
            return None;
        }
        let discriminant = discriminant.sqrt();
        let temp = (-b - discriminant) / a;
        if temp < t_max && temp > t_min {
            let pointing_at = ray.point_at(temp);
            let normal = (pointing_at - center).scalar_div(radius);
            let result = Some(HitRecord {
                t: temp,
                pointing_at,
                normal,
                material: self.material(),
            });
            return result;
        } else {
            let temp = (-b + discriminant) / a;
            if temp < t_max && temp > t_min {
                let pointing_at = ray.point_at(temp);
                return Some(HitRecord {
                    t: temp,
                    pointing_at,
                    normal: (pointing_at - center).scalar_div(radius),
                    material: self.material(),
                });
            }
        }
        None
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct StaticSphere {
    radius: f32,
    center: Vec3,
    material: Material,
}

impl StaticSphere {
    pub fn new(radius: f32, center: Vec3, material: Material) -> StaticSphere {
        StaticSphere {
            radius,
            center,
            material,
        }
    }
}

impl Sphere for StaticSphere {
    fn center(&self, _time: f32) -> Vec3 {
        self.center
    }

    fn radius(&self) -> f32 {
        self.radius
    }
    fn material(&self) -> Material {
        self.material.clone()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MovingSphere {
    radius: f32,
    material: Material,

    start: (f32, Vec3),
    end: (f32, Vec3),
}

impl MovingSphere {
    pub fn new(
        radius: f32,
        material: Material,
        start: (f32, Vec3),
        end: (f32, Vec3),
    ) -> MovingSphere {
        MovingSphere {
            radius,
            material,
            start,
            end,
        }
    }
}

impl Sphere for MovingSphere {
    fn center(&self, time: f32) -> Vec3 {
        let (time0, center0) = self.start;
        let (time1, center1) = self.end;
        center0 + (center1 - center0) * ((time - time0) / (time1 - time0))
    }

    fn radius(&self) -> f32 {
        self.radius
    }
    fn material(&self) -> Material {
        self.material.clone()
    }
}
