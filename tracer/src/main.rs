extern crate clap;
extern crate libtrace;
extern crate rand;
extern crate rayon;
use std::f32;

use libtrace::{
  material::{Dialectric, Lambertian, Metal},
  ppm, Camera, Hitable, Ray, Sphere, Vec3,
};

use rand::prelude::*;
use rayon::prelude::*;
use std::error::Error;
use std::io::Write;

use indicatif::{ProgressBar, ProgressStyle};
use std::fs;
use std::time::Instant;

/// interpolate between two vectors. t is an indicator
/// of how "far along" the interpolation should be. It should
/// be a floating point number in [0, 1]
fn lerp(start_value: Vec3, end_value: Vec3, t: f32) -> Vec3 {
  start_value * (1.0 - t) + end_value * t
}

fn color(ray: &Ray, world: impl Hitable, depth: i32) -> Vec3 {
  match world.hit(ray, 0.001, std::f32::MAX) {
    Some(hit_record) => {
      if depth < 50 {
        if let Some(scatter) = hit_record.material.scatter(ray, &hit_record) {
          return scatter.attenuation * color(&scatter.scatter, world, depth + 1);
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

fn random_scene() -> Vec<Box<dyn Hitable + Sync>> {
  let mut world: Vec<Box<dyn Hitable + Sync>> = Vec::new();
  world.push(Box::new(Sphere::new(
    1000.,
    (0., -1000., 0.).into(),
    Lambertian::new((0.5, 0.5, 0.5).into()).into(),
  )));

  let mut rng = rand::thread_rng();

  for a in -11..=10 {
    for b in -11..=10 {
      let a = a as f32;
      let b = b as f32;
      let material_type: f32 = rng.gen();
      let center = Vec3::new(a + 0.9 * rng.gen::<f32>(), 0.2, b + 0.9 * rng.gen::<f32>());
      if (center - Vec3::new(4., 0.2, 0.)).length() > 0.9 {
        if material_type < 0.8 {
          world.push(Box::new(Sphere::new(
            0.2,
            center,
            Lambertian::new(
              (
                rng.gen::<f32>() * rng.gen::<f32>(),
                rng.gen::<f32>() * rng.gen::<f32>(),
                rng.gen::<f32>() * rng.gen::<f32>(),
              )
                .into(),
            )
            .into(),
          )))
        } else if material_type < 0.95 {
          world.push(Box::new(Sphere::new(
            0.2,
            center,
            Metal::new(
              (
                0.5 * (1. + rng.gen::<f32>()),
                0.5 * (1. + rng.gen::<f32>()),
                0.5 * (1. + rng.gen::<f32>()),
              )
                .into(),
              0.5 * rng.gen::<f32>(),
            )
            .into(),
          )))
        } else {
          world.push(Box::new(Sphere::new(
            0.2,
            center,
            Dialectric::new(1.5).into(),
          )))
        }
      }
    }
  }

  world.push(Box::new(Sphere::new(
    1.0,
    (0., 1., 0.).into(),
    Dialectric::new(1.5).into(),
  )));

  world.push(Box::new(Sphere::new(
    1.0,
    (-4., 1., 0.).into(),
    Lambertian::new((0.4, 0.2, 0.1).into()).into(),
  )));

  world.push(Box::new(Sphere::new(
    1.0,
    (4., 1., 0.).into(),
    Metal::new((0.7, 0.6, 0.5).into(), 0.0).into(),
  )));

  world
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
  let num_samples = 50;

  let world = random_scene();
  let world = world.as_slice();

  let lookfrom = Vec3::new(13.0, 2.0, 3.0);
  let lookat = Vec3::new(0.0, 0.0, 0.0);
  let dist_to_focus = (lookfrom - lookat).length();
  let aperture = 0.05;

  let camera = Camera::new(
    lookfrom,
    lookat,
    Vec3::new(0.0, 1.0, 0.0),
    20.0,
    (width as f32) / (height as f32),
    aperture,
    dist_to_focus,
  );

  let mut pixels = Vec::with_capacity(height * width);
  for j in 0..height {
    let j = height - 1 - j;
    for i in 0..width {
      pixels.push((i, j));
    }
  }

  let bar = ProgressBar::new(pixels.len() as u64);
  bar.set_style(
    ProgressStyle::default_bar()
      .template("[{elapsed} elapsed] {wide_bar:.green/white} {percent}% [{eta} remaining]"),
  );

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
        samples.push(color(&r, world, 0));
      }
      let col: Vec3 = samples.into_iter().sum();
      let color = col / num_samples as f32;
      bar.inc(1);
      return color;
    })
    .collect();

  bar.finish();
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
