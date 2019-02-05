use crate::{Hitable, Vec3};
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
  pub slice: Option<ImageSlice>,
}

impl Image {
  pub fn num_pixels(&self) -> u32 {
    self.width() * self.height()
  }
  pub fn width(&self) -> u32 {
    self.width
  }
  pub fn height(&self) -> u32 {
    self
      .slice
      .as_ref()
      .map(|slice| slice.height)
      .unwrap_or(self.height)
  }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ImageSlice {
  pub top: u32,
  pub height: u32,
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
