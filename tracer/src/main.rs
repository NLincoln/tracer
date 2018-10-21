extern crate libtrace;
extern crate rand;
extern crate rayon;
use libtrace::{ppm, Camera, Hitable, Ray, Sphere, Vec3};
use rand::prelude::*;
use rayon::prelude::*;
use std::time::{Duration, Instant};

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
  let start = Instant::now();

  let width = 800;
  let height = 400;
  let num_samples = 100;

  print!("P3\n{} {}\n255\n", width, height);

  let world: Vec<Box<dyn Hitable + Sync>> = vec![
    Box::new(Sphere::new(0.5, Vec3::new(0., 0., -1.))),
    Box::new(Sphere::new(100., Vec3::new(0., -100.5, -1.))),
  ];
  let camera = Camera::new();

  let mut pixels: Vec<(usize, usize)> = Vec::new();
  pixels.reserve(height * width);
  for j in 0..height {
    let j = height - 1 - j;
    for i in 0..width {
      pixels.push((i, j));
    }
  }
  let result_image: Vec<_> = pixels
    .into_par_iter()
    .map(|(i, j)| {
      let mut rng = rand::thread_rng();
      let mut samples = Vec::new();
      samples.reserve(num_samples);
      for _ in 0..num_samples {
        let u = (i as f32 + rng.gen::<f32>()) / width as f32;
        let v = (j as f32 + rng.gen::<f32>()) / height as f32;
        let r = camera.get_ray(u, v);
        samples.push(color(&r, &world[..]));
      }
      let col = samples.iter().fold(Vec3::default(), |vec, item| vec + item);
      col / num_samples as f32
    }).collect();
  let duration = start.elapsed();

  eprintln!(
    "Took {}s",
    duration.as_secs() as f64 + duration.subsec_millis() as f64 * 1e-3
  );
  for pixel in result_image {
    println!("{}", ppm::format_as_color(&pixel));
  }
}
