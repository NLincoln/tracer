use crate::aabb::Aabb;
use crate::{HitRecord, Hitable, Ray};
use serde_derive::{Deserialize, Serialize};

use rand::Rng;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BvhNode {
    left: Box<Hitable>,
    right: Box<Hitable>,
    bbox: Aabb,
}

impl BvhNode {
    pub fn new(mut hitables: Vec<Hitable>, time: (f32, f32)) -> BvhNode {
        let axis = *rand::thread_rng().choose(&[0, 1, 2]).unwrap();

        hitables.sort_by(|a, b| {
            let a_bbox = a.bounding_box(time).min();
            let b_bbox = b.bounding_box(time).min();
            let a = a_bbox.as_slice()[axis];
            let b = b_bbox.as_slice()[axis];
            a.partial_cmp(&b).unwrap()
        });

        if hitables.len() == 1 {
            let node = hitables[0].clone();
            return BvhNode {
                left: node.clone().into(),
                right: node.clone().into(),
                bbox: node.bounding_box(time),
            };
        } else if hitables.len() == 2 {
            let left = hitables[0].clone();
            let right = hitables[1].clone();
            let bbox = Aabb::surrounding_box(left.bounding_box(time), right.bounding_box(time));

            return BvhNode {
                left: left.into(),
                right: right.into(),
                bbox,
            };
        }

        let last_half = hitables.split_off(hitables.len() / 2);
        let first_half = hitables;

        let left: Box<Hitable> = Hitable::from(BvhNode::new(first_half, time)).into();
        let right: Box<Hitable> = Hitable::from(BvhNode::new(last_half, time)).into();
        let bbox = Aabb::surrounding_box(left.bounding_box(time), right.bounding_box(time));

        BvhNode { left, right, bbox }
    }
    pub fn bounding_box(&self) -> Aabb {
        self.bbox.clone()
    }
    pub fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        if !self.bbox.hit(ray, t_min, t_max) {
            return None;
        }
        let hit_left = self.left.hit(ray, t_min, t_max);
        let hit_right = self.right.hit(ray, t_min, t_max);
        match (hit_left, hit_right) {
            (Some(left), Some(right)) => {
                if left.t < right.t {
                    Some(left)
                } else {
                    Some(right)
                }
            }
            (Some(left), None) => Some(left),
            (None, Some(right)) => Some(right),
            (None, None) => None,
        }
    }
}
