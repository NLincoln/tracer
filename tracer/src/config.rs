use serde::Deserialize;
use libtrace::Vec3;

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
  pub image: Image,
  pub camera: Camera
}

#[derive(Debug, Clone, Deserialize)]
pub struct Image {
  pub height: u32,
  pub width: u32,
  pub samples: u32
}

#[derive(Debug, Clone, Deserialize)]
pub struct Camera {
  pub look_from: Vec3,
  pub look_at: Vec3,
  pub aperture: f32,
  pub fov: f32
}
