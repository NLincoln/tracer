use std::fmt::{self, Debug};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

#[derive(PartialEq, Clone, Copy)]
pub struct Vec3([f32; 3]);

impl Debug for Vec3 {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "Vec3({}, {}, {})", self.x(), self.y(), self.z())
  }
}
impl Vec3 {
  /// Create a new vector from the given components
  pub fn new(x: f32, y: f32, z: f32) -> Vec3 {
    Vec3([x, y, z])
  }
  /// Creates a vector from an array slice
  ///
  /// Returns `None` with the following conditions when provided a slice with anything
  /// other than 3 elements
  pub fn from_slice(slice: &[f32]) -> Option<Vec3> {
    if slice.len() != 3 {
      return None;
    }

    return Some(Vec3([slice[0], slice[1], slice[2]]));
  }

  /// Creates a vector from an array slice, unchecked
  /// The difference between this and the checked version is that this
  /// doesn't do the bounds checking that the other one does.
  ///
  /// If the slice has a length < 3, this will panic
  pub unsafe fn from_slice_unchecked(slice: &[f32]) -> Vec3 {
    return Vec3([slice[0], slice[1], slice[2]]);
  }

  pub fn length(&self) -> f32 {
    return self.squared_length().sqrt();
  }
  pub fn squared_length(&self) -> f32 {
    return self.clone().dot(self);
  }

  /// Mutates the underlying vector and normalizes it, which means it
  /// transforms the vector from a vector of length n in some direction,
  /// to a vector of length 1 in the same direction
  ///
  /// If you need to do this immutably, you'll want to
  /// .clone() the vector first. We mutate instead by default because
  /// an immutable version of this method would clone anyway, and
  /// therefore leaving the choice to the user would be best.
  ///
  /// ```
  /// use libtrace::vec3::Vec3;
  ///
  /// let mut vec = Vec3::new(2., 3., 10.);
  /// vec.normalize();
  /// let len = vec.length();
  ///
  /// assert!(len > 0.99999);
  /// assert!(len < 1.00001);
  /// ```
  ///
  pub fn normalize(&mut self) {
    let len = self.length();
    *self.mut_x() /= len;
    *self.mut_y() /= len;
    *self.mut_z() /= len;
  }

  /// Multipy the contents of this vector by a scalar value
  pub fn scalar_mult_mut(&mut self, val: f32) {
    *self.mut_x() *= val;
    *self.mut_y() *= val;
    *self.mut_z() *= val;
  }

  pub fn scalar_mult(mut self, val: f32) -> Vec3 {
    self.scalar_mult_mut(val);
    return self;
  }

  pub fn scalar_div_mut(&mut self, val: f32) {
    *self.mut_x() /= val;
    *self.mut_y() /= val;
    *self.mut_z() /= val;
  }

  pub fn scalar_div(mut self, val: f32) -> Vec3 {
    self.scalar_div_mut(val);
    self
  }

  /// Returns the x component of the vector (or i-hat, or <1, 0, 0>)
  pub fn x(&self) -> f32 {
    self.0[0]
  }
  pub fn y(&self) -> f32 {
    self.0[1]
  }
  pub fn z(&self) -> f32 {
    self.0[2]
  }

  pub fn mut_x(&mut self) -> &mut f32 {
    &mut self.0[0]
  }
  pub fn mut_y(&mut self) -> &mut f32 {
    &mut self.0[1]
  }
  pub fn mut_z(&mut self) -> &mut f32 {
    &mut self.0[2]
  }

  pub fn dot(mut self, other: &Vec3) -> f32 {
    *self.mut_x() *= other.x();
    *self.mut_y() *= other.y();
    *self.mut_z() *= other.z();
    self.x() + self.y() + self.z()
  }
}

impl Default for Vec3 {
  fn default() -> Vec3 {
    Vec3([0., 0., 0.])
  }
}

impl Add for Vec3 {
  type Output = Vec3;
  #[inline]
  fn add(mut self, other: Vec3) -> Vec3 {
    self += other;
    self
  }
}

impl AddAssign for Vec3 {
  #[inline]
  fn add_assign(&mut self, rhs: Vec3) {
    *self.mut_x() += rhs.x();
    *self.mut_y() += rhs.y();
    *self.mut_z() += rhs.z();
  }
}

impl Sub for Vec3 {
  type Output = Vec3;
  #[inline]
  fn sub(mut self, other: Vec3) -> Vec3 {
    self -= other;
    self
  }
}

impl SubAssign for Vec3 {
  #[inline]
  fn sub_assign(&mut self, rhs: Vec3) {
    *self.mut_x() -= rhs.x();
    *self.mut_y() -= rhs.y();
    *self.mut_z() -= rhs.z();
  }
}

impl Mul for Vec3 {
  type Output = Vec3;
  #[inline]
  fn mul(self, other: Vec3) -> Vec3 {
    Vec3::new(
      self.x() * other.x(),
      self.y() * other.y(),
      self.z() * other.z(),
    )
  }
}

impl MulAssign for Vec3 {
  #[inline]
  fn mul_assign(&mut self, rhs: Vec3) {
    *self.mut_x() *= rhs.x();
    *self.mut_y() *= rhs.y();
    *self.mut_z() *= rhs.z();
  }
}

impl Div for Vec3 {
  type Output = Vec3;
  fn div(self, rhs: Vec3) -> Vec3 {
    Vec3::new(self.x() / rhs.x(), self.y() / rhs.y(), self.z() / rhs.z())
  }
}

impl DivAssign for Vec3 {
  fn div_assign(&mut self, rhs: Vec3) {
    *self.mut_x() /= rhs.x();
    *self.mut_y() /= rhs.y();
    *self.mut_z() /= rhs.z();
  }
}

impl From<(f32, f32, f32)> for Vec3 {
  fn from(triple: (f32, f32, f32)) -> Vec3 {
    Vec3::new(triple.0, triple.1, triple.2)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_ops() {
    assert_eq!(
      Vec3::new(1., 1., 1.) + Vec3::new(2., 2., 2.),
      Vec3::new(3., 3., 3.)
    );
    assert_eq!(
      Vec3::new(1., 1., 1.) * Vec3::new(2., 2., 2.),
      Vec3::new(2., 2., 2.)
    );
    let mut a = Vec3::new(1., 1., 1.);
    a *= Vec3::new(4., 5., 2.);
    assert_eq!(a, Vec3::new(4., 5., 2.));
  }
}
