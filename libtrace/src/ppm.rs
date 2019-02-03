use crate::vec3::Vec3;

pub fn format_as_color(vec: &Vec3) -> String {
  let transform = |color: f32| (color.sqrt() * 255.99) as u8;
  format!(
    "{} {} {}",
    transform(vec.x()),
    transform(vec.y()),
    transform(vec.z()),
  )
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
