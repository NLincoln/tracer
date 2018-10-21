extern crate libtrace;
use self::vec3::Vec3;
use libtrace::{ppm, ray, sphere, vec3};

fn color(ray: &ray::Ray) -> Vec3 {
  if ray.intersects_sphere(&sphere::Sphere::new(0.5, Vec3::new(0., 0., -1.))) {
    return Vec3::new(1., 0., 0.);
  }
  let mut unit_direction = ray.direction().clone();
  unit_direction.normalize();
  let t: f32 = 0.5 * (unit_direction.y() + 1.0);

  return Vec3::new(1.0, 1.0, 1.0).scalar_mult(1.0 - t) + Vec3::new(0.5, 0.7, 1.0).scalar_mult(t);
}

fn main() {
  let nx = 200;
  let ny = 100;

  print!("P3\n{} {}\n255\n", nx, ny);

  let lower_left_corner = Vec3::new(-2.0, -1.0, -1.0);
  let horizontal = Vec3::new(4.0, 0.0, 0.0);
  let vertical = Vec3::new(0.0, 2.0, 0.0);
  let origin = Vec3::new(0.0, 0.0, 0.0);

  for j in 0..ny {
    let j = ny - j;
    for i in 0..nx {
      let u = i as f32 / nx as f32;
      let v = j as f32 / ny as f32;
      let r = ray::Ray::new(
        origin.clone(),
        lower_left_corner.clone()
          + horizontal.clone().scalar_mult(u)
          + vertical.clone().scalar_mult(v),
      );
      let col = color(&r);

      println!("{}", ppm::format_as_color(&col));
    }
  }
}
