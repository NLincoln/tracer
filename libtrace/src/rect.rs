use crate::aabb::Aabb;
use crate::{HitRecord, Material, Ray, Vec3};
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "type")]
pub enum Rect {
    XY(XYRect),
    XZ(XZRect),
    YZ(YZRect),
}
impl Rect {
    pub fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        match self {
            Rect::XY(rect) => rect.hit(ray, t_min, t_max),
            Rect::YZ(rect) => rect.hit(ray, t_min, t_max),
            Rect::XZ(rect) => rect.hit(ray, t_min, t_max),
        }
    }
    pub fn bounding_box(&self, time: (f32, f32)) -> Aabb {
        match self {
            Rect::XY(rect) => rect.bounding_box(time),
            Rect::YZ(rect) => rect.bounding_box(time),
            Rect::XZ(rect) => rect.bounding_box(time),
        }
    }
}
impl From<XYRect> for Rect {
    fn from(rect: XYRect) -> Rect {
        Rect::XY(rect)
    }
}

impl From<XZRect> for Rect {
    fn from(rect: XZRect) -> Rect {
        Rect::XZ(rect)
    }
}
impl From<YZRect> for Rect {
    fn from(rect: YZRect) -> Rect {
        Rect::YZ(rect)
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct XYRect {
    x0: f32,
    x1: f32,
    y0: f32,
    y1: f32,
    plane_offset: f32,
    material: Material,
}

impl XYRect {
    pub fn new<M: Into<Material>>(
        x0: f32,
        x1: f32,
        y0: f32,
        y1: f32,
        plane_offset: f32,
        material: M,
    ) -> XYRect {
        XYRect {
            x0,
            x1,
            y0,
            y1,
            plane_offset,
            material: material.into(),
        }
    }

    pub fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let t = (self.plane_offset - ray.origin().z()) / ray.direction().z();
        if t < t_min || t > t_max {
            return None;
        }
        let x = ray.origin().x() + t * ray.direction().x();
        let y = ray.origin().y() + t * ray.direction().y();
        if x < self.x0 || x > self.x1 || y < self.y0 || y > self.y1 {
            return None;
        }
        Some(HitRecord {
            uv: (
                (x - self.x0) / (self.x1 - self.x0),
                (y - self.y0) / (self.y1 - self.y0),
            ),
            t,
            material: self.material.clone(),
            pointing_at: ray.point_at(t),
            normal: (0., 0., 1.).into(),
        })
    }
    pub fn bounding_box(&self, time: (f32, f32)) -> Aabb {
        Aabb::new(
            (self.x0, self.y0, self.plane_offset - 0.0001).into(),
            (self.x1, self.y1, self.plane_offset + 0.0001).into(),
        )
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct XZRect {
    x0: f32,
    x1: f32,
    z0: f32,
    z1: f32,
    plane_offset: f32,
    material: Material,
}

impl XZRect {
    pub fn new<M: Into<Material>>(
        x0: f32,
        x1: f32,
        z0: f32,
        z1: f32,
        plane_offset: f32,
        material: M,
    ) -> XZRect {
        XZRect {
            x0,
            x1,
            z0,
            z1,
            plane_offset,
            material: material.into(),
        }
    }

    pub fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let t = (self.plane_offset - ray.origin().y()) / ray.direction().y();
        if t < t_min || t > t_max {
            return None;
        }
        let x = ray.origin().x() + t * ray.direction().x();
        let z = ray.origin().z() + t * ray.direction().z();
        if x < self.x0 || x > self.x1 || z < self.z0 || z > self.z1 {
            return None;
        }
        Some(HitRecord {
            uv: (
                (x - self.x0) / (self.x1 - self.x0),
                (z - self.z0) / (self.z1 - self.z0),
            ),
            t,
            material: self.material.clone(),
            pointing_at: ray.point_at(t),
            normal: (0., 1., 0.).into(),
        })
    }
    pub fn bounding_box(&self, time: (f32, f32)) -> Aabb {
        Aabb::new(
            (self.x0, self.plane_offset - 0.0001, self.z0).into(),
            (self.x1, self.plane_offset + 0.0001, self.z1).into(),
        )
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct YZRect {
    y0: f32,
    y1: f32,
    z0: f32,
    z1: f32,
    plane_offset: f32,
    material: Material,
}

impl YZRect {
    pub fn new<M: Into<Material>>(
        y0: f32,
        y1: f32,
        z0: f32,
        z1: f32,
        plane_offset: f32,
        material: M,
    ) -> YZRect {
        YZRect {
            y0,
            y1,
            z0,
            z1,
            plane_offset,
            material: material.into(),
        }
    }

    pub fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let t = (self.plane_offset - ray.origin().x()) / ray.direction().x();
        if t < t_min || t > t_max {
            return None;
        }
        let y = ray.origin().y() + t * ray.direction().y();
        let z = ray.origin().z() + t * ray.direction().z();
        if y < self.y0 || y > self.y1 || z < self.z0 || y > self.z1 {
            return None;
        }
        Some(HitRecord {
            uv: (
                (y - self.y0) / (self.y1 - self.y0),
                (z - self.z0) / (self.z1 - self.z0),
            ),
            t,
            material: self.material.clone(),
            pointing_at: ray.point_at(t),
            normal: (1., 0., 0.).into(),
        })
    }
    pub fn bounding_box(&self, time: (f32, f32)) -> Aabb {
        Aabb::new(
            (self.plane_offset - 0.0001, self.y0, self.z0).into(),
            (self.plane_offset + 0.0001, self.y1, self.z1).into(),
        )
    }
}
