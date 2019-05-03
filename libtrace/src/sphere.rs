use crate::aabb::Aabb;
use crate::{HitRecord, Material, Ray, Vec3};
use serde_derive::{Deserialize, Serialize};

fn get_sphere_uv(p: Vec3) -> (f32, f32) {
    use std::f32::consts::{FRAC_PI_2, PI};

    let phi = p.z().atan2(p.x());
    let theta = p.y().asin();
    let u = 1. - (phi + PI) / (2. * PI);
    let v = (theta + FRAC_PI_2) / PI;
    (u, v)
}

pub trait Sphere {
    fn center(&self, time: f32) -> Vec3;
    fn radius(&self) -> f32;
    fn material(&self) -> Material;

    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let center = self.center(ray.time());
        let radius = self.radius();

        let oc = ray.origin() - center;
        let a = ray.direction().dot(ray.direction());
        let b = oc.dot(ray.direction());
        let c = oc.squared_length() - radius * radius;
        let discriminant = b * b - a * c;
        if discriminant < 0. {
            return None;
        }
        let discriminant = discriminant.sqrt();
        for discriminant_mul in [-1., 1.].iter() {
            let temp = (-b + *discriminant_mul * discriminant) / a;
            if temp > t_max || temp < t_min {
                continue;
            }
            let pointing_at = ray.point_at(temp);
            let normal = (pointing_at - center).scalar_div(radius);
            return Some(HitRecord {
                t: temp,
                pointing_at,
                normal,
                uv: get_sphere_uv((pointing_at - center) / radius),
                material: self.material(),
            });
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
    pub fn new<V: Into<Vec3>, M: Into<Material>>(
        radius: f32,
        center: V,
        material: M,
    ) -> StaticSphere {
        StaticSphere {
            radius,
            center: center.into(),
            material: material.into(),
        }
    }
    pub fn bounding_box(&self) -> Aabb {
        Aabb::new(
            self.center - Vec3::new(self.radius, self.radius, self.radius),
            self.center + Vec3::new(self.radius, self.radius, self.radius),
        )
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
    pub fn bounding_box(&self, (t0, t1): (f32, f32)) -> Aabb {
        let box0 = Aabb::new(
            self.center(t0) - Vec3::new(self.radius, self.radius, self.radius),
            self.center(t0) + Vec3::new(self.radius, self.radius, self.radius),
        );
        let box1 = Aabb::new(
            self.center(t1) - Vec3::new(self.radius, self.radius, self.radius),
            self.center(t1) + Vec3::new(self.radius, self.radius, self.radius),
        );
        Aabb::surrounding_box(box0, box1)
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
