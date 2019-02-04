use env_logger::{Builder, Env};
use lambda_http::{lambda, IntoResponse, Request};
use lambda_runtime::{error::HandlerError, Context};
use libtrace::{Camera, Vec3};
use rand::prelude::*;
use worker_shared::{Rendered, Scene};

fn main() {
  Builder::from_env(
    Env::default()
      .filter("TRACER_LOG")
      .default_filter_or("warn"),
  )
  .init();
  lambda!(handler)
}

fn handler(request: Request, _: Context) -> Result<impl IntoResponse, HandlerError> {
  log::info!("Received request");
  let body = request.body();

  let scene: Scene = serde_json::from_slice(body)?;

  let width = scene.image.width;
  let height = scene.image.height;
  let num_samples = scene.image.samples;

  let dist_to_focus = (scene.camera.look_from - scene.camera.look_at).length();

  let camera = Camera::new(
    scene.camera.look_from,
    scene.camera.look_at,
    Vec3::new(0.0, 1.0, 0.0),
    scene.camera.fov,
    (width as f32) / (height as f32),
    scene.camera.aperture,
    dist_to_focus,
  );

  let mut pixels = Vec::with_capacity((height * width) as usize);
  for j in 0..height {
    let j = height - 1 - j;
    for i in 0..width {
      pixels.push((i, j));
    }
  }

  let result_image: Vec<_> = pixels
    .into_iter()
    .map(|(i, j)| {
      let width = width as f32;
      let height = height as f32;
      let i = i as f32;
      let j = j as f32;

      let mut rng = rand::thread_rng();
      let mut samples = Vec::new();
      samples.reserve(num_samples as usize);
      for _ in 0..num_samples {
        // U and V are the actual coordinates on the
        // image plane we are targeting.
        // the rand adds a tiny bit of "wobble"
        // to our sample, which is good for sampling
        let u = (i + rng.gen::<f32>()) / width;
        let v = (j + rng.gen::<f32>()) / height;
        let r = camera.get_ray(u, v);
        samples.push(libtrace::color(&scene.sky_color, &r, &scene.objects, 0));
      }
      let col: Vec3 = samples.into_iter().sum();
      let color = col / num_samples as f32;
      return libtrace::ppm::to_color(&color);
    })
    .collect();

  Ok(serde_json::to_string(&Rendered {
    image: scene.image,
    pixels: result_image,
  })?)
}
