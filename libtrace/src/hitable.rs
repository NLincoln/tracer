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
pub enum Hitable {
    StaticSphere(StaticSphere),
    MovingSphere(MovingSphere),
    BvhNode(BvhNode),
    List(Vec<Hitable>),
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

impl From<Vec<Hitable>> for Hitable {
    #[inline]
    fn from(list: Vec<Hitable>) -> Hitable {
        Hitable::List(list)
    }
}

impl Hitable {
    pub fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        match self {
            Hitable::StaticSphere(s) => s.hit(ray, t_min, t_max),
            Hitable::MovingSphere(s) => s.hit(ray, t_min, t_max),
            Hitable::BvhNode(node) => node.hit(ray, t_min, t_max),
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
    pub fn bounding_box(&self, t0: f32, t1: f32) -> Aabb {
        match self {
            Hitable::StaticSphere(s) => s.bounding_box(t0, t1),
            Hitable::MovingSphere(s) => s.bounding_box(t0, t1),
            Hitable::BvhNode(node) => node.bounding_box(),
            Hitable::List(list) => {
                let init = list[0].bounding_box(t0, t1);
                list[1..].into_iter().fold(init, |prev, curr| {
                    Aabb::surrounding_box(prev, curr.bounding_box(t0, t1))
                })
            }
        }
    }
}
