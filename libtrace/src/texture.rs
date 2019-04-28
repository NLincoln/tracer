use crate::Vec3;
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "type")]
pub enum Texture {
    Color(Color),
}
impl Texture {
    fn value(&self, u: f32, v: f32) -> Vec3 {
        match self {
            Texture::Color(color) => color.value(u, v),
        }
    }
}

impl From<Color> for Texture {
    fn from(color: Color) -> Texture {
        Texture::Color(color)
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Color {
    color: Vec3,
}

impl Color {
    fn value(&self, u: f32, v: f32) -> Vec3 {
        self.color
    }
}
