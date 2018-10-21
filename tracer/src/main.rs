extern crate libtrace;
use libtrace::{ppm, Hitable, Ray, Sphere, Vec3};

fn color(ray: &Ray, world: impl Hitable) -> Vec3 {
  match world.hit(ray, 0.0, std::f32::MAX) {
    Some(hit_record) => Vec3::new(
      hit_record.normal.x() + 1.,
      hit_record.normal.y() + 1.,
      hit_record.normal.z() + 1.,
    ).scalar_mult(0.5),
    None => {
      let mut unit_direction = ray.direction().clone();
      unit_direction.normalize();
      let t: f32 = 0.5 * (unit_direction.y() + 1.0);

      Vec3::new(1.0, 1.0, 1.0).scalar_mult(1.0 - t) + Vec3::new(0.5, 0.7, 1.0).scalar_mult(t)
    }
  }
}

fn main() {
  let nx = 400;
  let ny = 200;

  print!("P3\n{} {}\n255\n", nx, ny);

  let lower_left_corner = Vec3::new(-2.0, -1.0, -1.0);
  let horizontal = Vec3::new(4.0, 0.0, 0.0);
  let vertical = Vec3::new(0.0, 2.0, 0.0);
  let origin = Vec3::new(0.0, 0.0, 0.0);

  let world: Vec<Box<dyn Hitable>> = vec![
    Box::new(Sphere::new(0.5, Vec3::new(0., 0., -1.))),
    Box::new(Sphere::new(100., Vec3::new(0., -100.5, -1.))),
  ];

  for j in 0..ny {
    let j = ny - 1 - j;
    for i in 0..nx {
      let u = i as f32 / nx as f32;
      let v = j as f32 / ny as f32;
      let r = Ray::new(
        origin.clone(),
        lower_left_corner.clone()
          + horizontal.clone().scalar_mult(u)
          + vertical.clone().scalar_mult(v),
      );
      let col = color(&r, &world[..]);

      println!("{}", ppm::format_as_color(&col));
    }
  }
}
