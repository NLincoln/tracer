extern crate clap;
extern crate libtrace;
extern crate rand;
extern crate rayon;

use libtrace::{ppm, Camera, Hitable, Ray, Sphere, Vec3};
use rand::prelude::*;
use rayon::prelude::*;
use std::error::Error;
use std::io::Write;

use std::fs;
use std::time::Instant;

/// interpolate between two vectors. t is an indicator
/// of how "far along" the interpolation should be. It should
/// be a floating point number in [0, 1]
fn lerp(start_value: Vec3, end_value: Vec3, t: f32) -> Vec3 {
  start_value * (1.0 - t) + end_value * t
}

fn color(ray: &Ray, world: impl Hitable) -> Vec3 {
  match world.hit(ray, 0.0, std::f32::MAX) {
    Some(hit_record) => Vec3::new(
      hit_record.normal.x() + 1.,
      hit_record.normal.y() + 1.,
      hit_record.normal.z() + 1.,
    )
    .scalar_mult(0.5),
    None => {
      /*
       * We didn't get a hit! This means that
       * we need to calculate the color
       * of the sky instead.
       */
      let sky_color = Vec3::new(0.5, 0.7, 1.0);

      // What direction was this ray traveling?
      // we want this to be a unit vector so that our
      // y component is between -1 and 1
      let unit_direction = ray.direction().into_normalized();

      // So since -1 < y() < 1, we want it to be 0 < y < 1,
      // the easest way to do that is to add one to y() and divide.
      // height represents how high on the screen this ray was
      let height: f32 = 0.5 * (unit_direction.y() + 1.0);

      lerp(Vec3::new(1., 1., 1.), sky_color, height)
    }
  }
}

fn main() -> Result<(), Box<dyn Error>> {
  let matches = clap::App::new("Tracer")
    .version("0.1.0")
    .about("Ray traces an image")
    .arg(
      clap::Arg::with_name("output")
        .short("o")
        .value_name("FILE")
        .takes_value(true)
        .default_value("image.ppm"),
    )
    .get_matches();

  let start = Instant::now();

  let width = 800;
  let height = 400;
  let num_samples = 100;

  let world: &[Box<dyn Hitable + Sync>] = &[
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
      let width = width as f32;
      let height = height as f32;
      let i = i as f32;
      let j = j as f32;

      let mut rng = rand::thread_rng();
      let mut samples = Vec::new();
      samples.reserve(num_samples);
      for _ in 0..num_samples {
        // U and V are the actual coordinates on the
        // image plane we are targeting.
        // the rand adds a tiny bit of "wobble"
        // to our sample, which is good for sampling
        let u = (i + rng.gen::<f32>()) / width;
        let v = (j + rng.gen::<f32>()) / height;
        let r = camera.get_ray(u, v);
        samples.push(color(&r, world));
      }
      let col: Vec3 = samples.into_iter().sum();
      col / num_samples as f32
    })
    .collect();
  let duration = start.elapsed();

  eprintln!(
    "Took {}s",
    duration.as_secs() as f64 + duration.subsec_millis() as f64 * 1e-3
  );
  let mut output = fs::OpenOptions::new()
    .create(true)
    .write(true)
    .truncate(true)
    .open(matches.value_of("output").unwrap())?;

  write!(output, "P3\n{} {}\n255\n", width, height)?;

  for pixel in result_image {
    writeln!(output, "{}", ppm::format_as_color(&pixel))?;
  }

  Ok(())
}
