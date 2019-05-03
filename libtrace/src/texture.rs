pub use crate::perlin::NoiseTexture;
use crate::Vec3;
use image::{DynamicImage, GenericImageView};
use serde_derive::{Deserialize, Serialize};
use std::fmt::{self, Debug};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "type")]
pub enum Texture {
    Color(Color),
    CheckerBoard(Box<CheckerBoard>),
    RecursiveCheckerboard(Box<RecursiveCheckerboard>),
    Noise(NoiseTexture),
    Image(Image),
}

impl Texture {
    pub fn value(&self, u: f32, v: f32, p: Vec3) -> Vec3 {
        match self {
            Texture::Color(color) => color.value(),
            Texture::CheckerBoard(checker) => checker.value(u, v, p),
            Texture::RecursiveCheckerboard(checker) => checker.value(u, v, p),
            Texture::Noise(noise) => noise.value(u, v, p),
            Texture::Image(image) => image.value(u, v, p),
        }
    }
}

impl From<Color> for Texture {
    fn from(color: Color) -> Texture {
        Texture::Color(color)
    }
}

impl From<CheckerBoard> for Texture {
    fn from(color: CheckerBoard) -> Texture {
        Texture::CheckerBoard(color.into())
    }
}

impl From<RecursiveCheckerboard> for Texture {
    fn from(color: RecursiveCheckerboard) -> Texture {
        Texture::RecursiveCheckerboard(color.into())
    }
}
impl From<NoiseTexture> for Texture {
    fn from(color: NoiseTexture) -> Texture {
        Texture::Noise(color)
    }
}

impl From<Image> for Texture {
    fn from(color: Image) -> Texture {
        Texture::Image(color)
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CheckerBoard {
    odd: Texture,
    even: Texture,
    scale: f32,
}

impl CheckerBoard {
    pub fn new<O: Into<Texture>, E: Into<Texture>>(scale: f32, odd: O, even: E) -> CheckerBoard {
        CheckerBoard {
            scale,
            odd: odd.into(),
            even: even.into(),
        }
    }
    pub fn value(&self, u: f32, v: f32, p: Vec3) -> Vec3 {
        let is_odd = (self.scale * p.x()).sin().powi(2) * (self.scale * p.z()).sin();
        if is_odd < 0. {
            self.odd.value(u, v, p)
        } else {
            self.even.value(u, v, p)
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RecursiveCheckerboard {
    scale: f32,
    odd: Texture,
    even: Texture,
}

impl RecursiveCheckerboard {
    pub fn new<O: Into<Texture>, E: Into<Texture>>(
        scale: f32,
        odd: O,
        even: E,
    ) -> RecursiveCheckerboard {
        RecursiveCheckerboard {
            scale,
            odd: odd.into(),
            even: even.into(),
        }
    }
    fn value(&self, u: f32, v: f32, p: Vec3) -> Vec3 {
        self.value_recurse(self.scale, u, v, p)
    }
    fn value_recurse(&self, scale: f32, u: f32, v: f32, p: Vec3) -> Vec3 {
        let is_odd = (scale * p.x()).sin() * (scale * p.y()).sin() * (scale * p.z()).sin();
        if is_odd < 0. {
            if scale > 100. {
                self.odd.value(u, v, p)
            } else {
                self.value_recurse(scale * 8., u, v, p)
            }
        } else {
            self.even.value(u, v, p)
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Color {
    color: Vec3,
}

impl Color {
    fn value(&self) -> Vec3 {
        self.color
    }
    pub fn new<V: Into<Vec3>>(color: V) -> Color {
        Color {
            color: color.into(),
        }
    }
}

#[derive(Clone)]
pub struct Image {
    image: DynamicImage,
}

impl Debug for Image {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Image({}x{})", self.image.width(), self.image.height())
    }
}

impl serde::Serialize for Image {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        unimplemented!()
    }
}
impl<'de> serde::Deserialize<'de> for Image {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        unimplemented!()
    }
}
pub fn clamp<T: PartialOrd>(input: T, min: T, max: T) -> T {
    debug_assert!(min <= max, "min must be less than or equal to max");
    if input < min {
        min
    } else if input > max {
        max
    } else {
        input
    }
}
impl Image {
    pub fn new(image: DynamicImage) -> Image {
        Image { image }
    }

    fn value(&self, u: f32, v: f32, p: Vec3) -> Vec3 {
        let i = u * self.image.width() as f32;
        let j = (1. - v) * self.image.height() as f32 - 0.001;
        let i = clamp(i, 0., self.image.width() as f32);
        let j = clamp(j, 0., self.image.height() as f32);

        let rgb = self.image.get_pixel(i as u32, j as u32);

        let [r, g, b, _] = rgb.data;
        Vec3::new(r as f32, g as f32, b as f32).apply(|v| v / 255.0)
    }
}
