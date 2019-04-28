use crate::{Ray, Vec3};
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Aabb {
    min: Vec3,
    max: Vec3,
}

#[inline]
fn fmin(a: f32, b: f32) -> f32 {
    if a < b {
        a
    } else {
        b
    }
}

#[inline]
fn fmax(a: f32, b: f32) -> f32 {
    if a > b {
        a
    } else {
        b
    }
}

impl Aabb {
    pub fn new(min: Vec3, max: Vec3) -> Aabb {
        Aabb { min, max }
    }

    pub fn surrounding_box(a: Aabb, b: Aabb) -> Aabb {
        let small = Vec3::new(
            fmin(a.min().x(), b.min().x()),
            fmin(a.min().y(), b.min().y()),
            fmin(a.min().z(), b.min().z()),
        );
        let big = Vec3::new(
            fmax(a.max().x(), b.max().x()),
            fmax(a.max().y(), b.max().y()),
            fmax(a.max().z(), b.max().z()),
        );
        Aabb::new(small, big)
    }

    pub fn min(&self) -> Vec3 {
        self.min
    }

    pub fn max(&self) -> Vec3 {
        self.max
    }

    pub fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> bool {
        for a in 0..3 {
            let inv_d = 1.0 / ray.direction().as_slice()[a];
            let mut t0 = (self.min.as_slice()[a] - ray.origin().as_slice()[a]) * inv_d;
            let mut t1 = (self.max.as_slice()[a] - ray.origin().as_slice()[a]) * inv_d;
            if inv_d < 0.0 {
                std::mem::swap(&mut t0, &mut t1);
            }
            let t_min = if t0 > t_min { t0 } else { t_min };
            let t_max = if t1 <= t_max { t1 } else { t_max };

            if t_max <= t_min {
                return false;
            }
        }
        true
    }
}
