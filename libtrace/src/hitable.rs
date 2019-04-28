use crate::aabb::Aabb;
use crate::bvh::BvhNode;
use crate::sphere::Sphere;
use crate::{Material, MovingSphere, Ray, StaticSphere, Vec3};
use serde_derive::{Deserialize, Serialize};

#[derive(Debug)]
pub struct HitRecord {
    pub t: f32,
    pub pointing_at: Vec3,
    pub normal: Vec3,
    pub material: Material,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Hitable {
    StaticSphere(StaticSphere),
    MovingSphere(MovingSphere),
    BvhNode(BvhNode),
}

impl From<StaticSphere> for Hitable {
    #[inline]
    fn from(sphere: StaticSphere) -> Hitable {
        Hitable::StaticSphere(sphere)
    }
}
impl From<MovingSphere> for Hitable {
    #[inline]
    fn from(sphere: MovingSphere) -> Hitable {
        Hitable::MovingSphere(sphere)
    }
}
impl From<BvhNode> for Hitable {
    #[inline]
    fn from(node: BvhNode) -> Hitable {
        Hitable::BvhNode(node)
    }
}

impl Hitable {
    #[inline]
    pub fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        match self {
            Hitable::StaticSphere(s) => s.hit(ray, t_min, t_max),
            Hitable::MovingSphere(s) => s.hit(ray, t_min, t_max),
            Hitable::BvhNode(node) => node.hit(ray, t_min, t_max),
        }
    }
    #[inline]
    pub fn bounding_box(&self, time: (f32, f32)) -> Aabb {
        match self {
            Hitable::StaticSphere(s) => s.bounding_box(),
            Hitable::MovingSphere(s) => s.bounding_box(time),
            Hitable::BvhNode(node) => node.bounding_box(),
        }
    }
}
