extern crate libtrace;
extern crate rand;
use libtrace::{ppm, Camera, Hitable, Ray, Sphere, Vec3};
use rand::prelude::*;

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
  let width = 200;
  let height = 100;
  let num_samples = 100;

  print!("P3\n{} {}\n255\n", width, height);

  let world: Vec<Box<dyn Hitable>> = vec![
    Box::new(Sphere::new(0.5, Vec3::new(0., 0., -1.))),
    Box::new(Sphere::new(100., Vec3::new(0., -100.5, -1.))),
  ];
  let camera = Camera::new();
  let mut rng = rand::thread_rng();

  for j in 0..height {
    let j = height - 1 - j;
    for i in 0..width {
      let mut samples = Vec::new();
      samples.reserve(num_samples);
      for _ in 0..num_samples {
        let u = (i as f32 + rng.gen::<f32>()) / width as f32;
        let v = (j as f32 + rng.gen::<f32>()) / height as f32;
        let r = camera.get_ray(u, v);
        samples.push(color(&r, &world[..]))
      }
      let col = samples.iter().fold(Vec3::default(), |vec, item| vec + item);
      let col = col / num_samples as f32;
      println!("{}", ppm::format_as_color(&col));
    }
  }
}
