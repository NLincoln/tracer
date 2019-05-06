mod aabb;
mod bvh;
mod camera;
mod hitable;
pub mod material;
mod perlin;
pub mod ppm;
mod ray;
pub mod rect;
pub mod renderer;
pub mod scene;
mod sphere;
pub mod texture;
mod vec3;

pub use bvh::BvhNode;
pub use camera::Camera;
pub use hitable::NormalFlipper;
pub use hitable::{HitRecord, Hitable};
pub use material::{Material, Scatter};
pub use ray::Ray;
pub use sphere::{MovingSphere, StaticSphere};
pub use vec3::Vec3;

/// interpolate between two vectors. t is an indicator
/// of how "far along" the interpolation should be. It should
/// be a floating point number in [0, 1]
#[inline]
pub fn lerp(start_value: Vec3, end_value: Vec3, t: f32) -> Vec3 {
    start_value * (1.0 - t) + end_value * t
}

pub fn color(ray: &Ray, world: &Hitable, depth: i32) -> Vec3 {
    match world.hit(ray, 0.001, std::f32::MAX) {
        Some(hit_record) => {
            let emitted = hit_record
                .material
                .emitted(hit_record.uv, hit_record.pointing_at);
            if depth < 50 {
                if let Some(scatter) = hit_record.material.scatter(ray, &hit_record) {
                    return scatter.attenuation * color(&scatter.scatter, world, depth + 1);
                }
            }
            emitted
        }
        None => 0f32.into(),
    }
}
