use crate::Vec3;
use std::fmt::{self, Debug};

#[derive(Clone, Default)]
pub struct Ray {
    time: f32,
    origin: Vec3,
    direction: Vec3,
}

impl Debug for Ray {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Ray({:?} + t * {:?})", self.origin, self.direction)
    }
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3, time: f32) -> Ray {
        Ray {
            origin,
            direction,
            time,
        }
    }
    #[inline]
    pub fn origin(&self) -> Vec3 {
        self.origin
    }
    #[inline]
    pub fn direction(&self) -> Vec3 {
        self.direction
    }

    pub fn time(&self) -> f32 {
        self.time
    }

    #[inline]
    pub fn point_at(&self, t: f32) -> Vec3 {
        self.origin + self.direction.scalar_mult(t)
    }
}
