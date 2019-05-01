use crate::aabb::Aabb;
use crate::sphere::Sphere;
use crate::{BvhNode, Material, MovingSphere, Ray, StaticSphere, Vec3};
use serde_derive::{Deserialize, Serialize};

#[derive(Debug)]
pub struct HitRecord {
    pub t: f32,
    pub pointing_at: Vec3,
    pub normal: Vec3,
    pub material: Material,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HitableList {
    items: Vec<Hitable>,
}

impl From<Vec<Hitable>> for HitableList {
    fn from(items: Vec<Hitable>) -> HitableList {
        HitableList { items }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Hitable {
    StaticSphere(StaticSphere),
    MovingSphere(MovingSphere),
    BvhNode(BvhNode),
    List(HitableList),
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
            Hitable::List(HitableList { items: list }) => {
                let mut closest_so_far = t_max;
                let mut result: Option<HitRecord> = None;
                for hitable in list.iter() {
                    if let Some(hit_record) = hitable.hit(ray, t_min, closest_so_far) {
                        closest_so_far = hit_record.t;
                        result = Some(hit_record);
                    }
                }
                result
            }
        }
    }
    #[inline]
    pub fn bounding_box(&self, time: (f32, f32)) -> Aabb {
        match self {
            Hitable::StaticSphere(s) => s.bounding_box(),
            Hitable::MovingSphere(s) => s.bounding_box(time),
            Hitable::BvhNode(node) => node.bounding_box(),
            Hitable::List(HitableList { items }) => {
                let init = items[0].bounding_box(time);
                items[1..].iter().fold(init, |prev, curr| {
                    Aabb::surrounding_box(prev, curr.bounding_box(time))
                })
            }
        }
    }
}
