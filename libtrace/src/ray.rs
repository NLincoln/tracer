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
        return self.origin.clone() + self.direction.clone().scalar_mult(t);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_ray_debug() {
        assert_eq!(
            format!(
                "{:?}",
                Ray::new(Vec3::new(1., 2., 3.), Vec3::new(4., 5., 6.))
            ),
            "Ray(Vec3(1, 2, 3) + t * Vec3(4, 5, 6))".to_string()
        );
    }
}
