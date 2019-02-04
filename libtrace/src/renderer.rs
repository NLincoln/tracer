use crate::{scene::Scene, Camera, Vec3};
use rand::prelude::*;
use std::error::Error;
use std::io::{BufWriter, Write};

/// A trait to help you define your own renderer.
/// Most of the guts of rendering are already provided for you, but
/// you are free to customize those methods as well.
pub trait Renderer {
  fn scene(&self) -> &Scene;
  fn camera(&self, scene: &Scene) -> Camera {
    let width = scene.image.width;
    let height = scene.image.height;
    let dist_to_focus = (scene.camera.look_from - scene.camera.look_at).length();

    Camera::new(
      scene.camera.look_from,
      scene.camera.look_at,
      Vec3::new(0.0, 1.0, 0.0),
      scene.camera.fov,
      (width as f32) / (height as f32),
      scene.camera.aperture,
      dist_to_focus,
    )
  }

  fn write_image(
    &self,
    buffer: &mut impl Write,
    pixels: &[(u8, u8, u8)],
  ) -> Result<(), Box<dyn Error>> {
    use png::HasParameters;

    let scene = self.scene();

    let writer = BufWriter::new(buffer);
    let mut encoder = png::Encoder::new(writer, scene.image.width, scene.image.height);
    encoder.set(png::ColorType::RGBA).set(png::BitDepth::Eight);
    let mut writer = encoder.write_header()?;
    let mut image_data = Vec::with_capacity(pixels.len() * 4);

    for pixel in pixels {
      image_data.push(pixel.0);
      image_data.push(pixel.1);
      image_data.push(pixel.2);
      image_data.push(255);
    }

    writer.write_image_data(&image_data)?;
    Ok(())
  }

  fn get_pixels_to_render(&self, scene: &Scene) -> Vec<(u32, u32)> {
    let width = scene.image.width;
    let height = scene.image.height;

    let mut pixels = Vec::with_capacity((height * width) as usize);

    for j in 0..height {
      let j = height - 1 - j;
      for i in 0..width {
        pixels.push((i, j));
      }
    }
    pixels
  }

  fn render(&self) -> Vec<(u8, u8, u8)> {
    let scene = self.scene();
    let camera = self.camera(&scene);

    self
      .get_pixels_to_render(&scene)
      .into_iter()
      .map(|(i, j)| self.render_pixel(&camera, (i, j), &scene))
      .collect()
  }

  fn render_pixel(&self, camera: &Camera, location: (u32, u32), scene: &Scene) -> (u8, u8, u8) {
    let width = scene.image.width as f32;
    let height = scene.image.height as f32;
    let num_samples = scene.image.samples;

    let i = location.0 as f32;
    let j = location.1 as f32;

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
      samples.push(crate::color(&scene.sky_color, &r, &scene.objects, 0));
    }
    let col: Vec3 = samples.into_iter().sum();
    let color = col / num_samples as f32;
    let color = crate::ppm::to_color(&color);
    self.on_pixel_rendered(location, color);
    return color;
  }
  fn on_pixel_rendered(&self, location: (u32, u32), color: (u8, u8, u8)) {}
}
