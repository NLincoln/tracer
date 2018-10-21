use std::fmt::{self, Display};

#[derive(Debug, PartialEq, Clone)]
pub struct Color {
  pub red: f32,
  pub green: f32,
  pub blue: f32,
}

impl Display for Color {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let transform = |color: f32| (color * 255.99) as u8;
    write!(
      f,
      "{} {} {}",
      transform(self.red),
      transform(self.green),
      transform(self.blue)
    )
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn test_format_color() {
    let color = Color {
      red: 0.,
      green: 0.5,
      blue: 1.,
    };
    assert_eq!(format!("{}", color), "0 127 255")
  }
}
