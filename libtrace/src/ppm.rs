use crate::vec3::Vec3;

fn transform(color: f32) -> u8 {
  (color.sqrt() * 255.99) as u8
}

pub fn format_as_color(vec: &Vec3) -> String {
  let color = to_color(vec);

  format!("{} {} {}", color.0, color.1, color.2,)
}

pub fn to_color(vec: &Vec3) -> (u8, u8, u8) {
  (transform(vec.x()), transform(vec.y()), transform(vec.z()))
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn test_format_color() {
    // 181 with gamma
    assert_eq!(format_as_color(&Vec3::new(0., 0.5, 1.)), "0 181 255")
  }
}
