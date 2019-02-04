mod camera;
mod hitable;
pub mod material;
pub mod ppm;
mod ray;
mod sphere;
mod vec3;

pub use camera::Camera;
pub use hitable::{HitRecord, Hitable};
pub use material::{Material, Scatter};
pub use ray::Ray;
pub use sphere::Sphere;
pub use vec3::Vec3;

/// interpolate between two vectors. t is an indicator
/// of how "far along" the interpolation should be. It should
/// be a floating point number in [0, 1]
pub fn lerp(start_value: Vec3, end_value: Vec3, t: f32) -> Vec3 {
  start_value * (1.0 - t) + end_value * t
}


pub fn color(sky_color: &Vec3, ray: &Ray, world: &Hitable, depth: i32) -> Vec3 {
  match world.hit(ray, 0.001, std::f32::MAX) {
    Some(hit_record) => {
      if depth < 50 {
        if let Some(scatter) = hit_record.material.scatter(ray, &hit_record) {
          return scatter.attenuation * color(sky_color, &scatter.scatter, world, depth + 1);
        }
      }
      return Vec3::default();
    }
    None => {
      /*
       * We didn't get a hit! This means that
       * we need to calculate the color
       * of the sky instead.
       */

      // What direction was this ray traveling?
      // we want this to be a unit vector so that our
      // y component is between -1 and 1
      let unit_direction = ray.direction().into_normalized();

      // So since -1 < y() < 1, we want it to be 0 < y < 1,
      // the easest way to do that is to add one to y() and divide.
      // height represents how high on the screen this ray was
      let height: f32 = 0.5 * (unit_direction.y() + 1.0);

      lerp(Vec3::new(1., 1., 1.), *sky_color, height)
    }
  }
}
