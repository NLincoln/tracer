use libtrace::{Hitable, Vec3};
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Scene {
  pub image: Image,
  pub camera: Camera,
  pub sky_color: Vec3,
  pub objects: Hitable,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Image {
  pub height: u32,
  pub width: u32,
  pub samples: u32,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Camera {
  pub look_from: Vec3,
  pub look_at: Vec3,
  pub aperture: f32,
  pub fov: f32,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Rendered {
  /// The image that was being rendered
  pub image: Image,
  /// The pixel data
  pub pixels: Vec<(u8, u8, u8)>,
}
