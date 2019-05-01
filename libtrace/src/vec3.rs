use serde_derive::{Deserialize, Serialize};
use std::fmt::{self, Debug};
use std::iter::Sum;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

#[derive(PartialEq, Clone, Copy, Serialize, Deserialize)]
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

    pub fn apply<F>(self, mut f: F) -> Vec3
    where
        F: FnMut(f32) -> f32,
    {
        Vec3::new(f(self.x()), f(self.y()), f(self.z()))
    }

    pub fn to_tuple(self) -> (f32, f32, f32) {
        (self.x(), self.y(), self.z())
    }

    pub fn to_tuple_and<T, F>(self, mut f: F) -> (T, T, T)
    where
        F: FnMut(f32) -> T,
    {
        let (x, y, z) = self.to_tuple();
        (f(x), f(y), f(z))
    }

    pub fn as_slice(&self) -> &[f32] {
        &self.0
    }
    /// Creates a random vector
    pub fn random_in_unit_circle() -> Vec3 {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        loop {
            let vec = Vec3::new(rng.gen(), rng.gen(), rng.gen()) * 2. - Vec3::new(1., 1., 1.);
            if vec.squared_length() <= 1. {
                return vec;
            }
        }
    }
    /// Creates a vector from an array slice
    ///
    /// Returns `None` with the following conditions when provided a slice with anything
    /// other than 3 elements
    pub fn from_slice(slice: &[f32]) -> Option<Vec3> {
        if slice.len() != 3 {
            return None;
        }

        Some(Vec3([slice[0], slice[1], slice[2]]))
    }

    /// Creates a vector from an array slice, unchecked
    /// The difference between this and the checked version is that this
    /// doesn't do the bounds checking that the other one does.
    ///
    /// If the slice has a length < 3, this will panic
    pub unsafe fn from_slice_unchecked(slice: &[f32]) -> Vec3 {
        Vec3([slice[0], slice[1], slice[2]])
    }

    pub fn length(&self) -> f32 {
        self.squared_length().sqrt()
    }
    pub fn squared_length(self) -> f32 {
        self.dot(self)
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
    /// use libtrace::Vec3;
    ///
    /// let mut vec = Vec3::new(2., 3., 10.);
    /// vec.normalize();
    /// let len = vec.length();
    ///
    /// assert!(len > 0.99999);
    /// assert!(len < 1.00001);
    /// ```
    ///
    #[inline]
    pub fn normalize(&mut self) {
        let len = self.length();
        *self.mut_x() /= len;
        *self.mut_y() /= len;
        *self.mut_z() /= len;
    }

    #[inline]
    pub fn into_normalized(self) -> Vec3 {
        self / self.length()
    }

    /// Multipy the contents of this vector by a scalar value
    #[inline]
    pub fn scalar_mult_mut(&mut self, val: f32) {
        *self.mut_x() *= val;
        *self.mut_y() *= val;
        *self.mut_z() *= val;
    }

    #[inline]
    pub fn scalar_mult(mut self, val: f32) -> Vec3 {
        self.scalar_mult_mut(val);
        self
    }

    #[inline]
    pub fn scalar_div_mut(&mut self, val: f32) {
        *self.mut_x() /= val;
        *self.mut_y() /= val;
        *self.mut_z() /= val;
    }

    #[inline]
    pub fn scalar_div(mut self, val: f32) -> Vec3 {
        self.scalar_div_mut(val);
        self
    }

    /// Returns the x component of the vector (or i-hat, or <1, 0, 0>)
    #[inline]
    pub fn x(&self) -> f32 {
        self.0[0]
    }
    #[inline]
    pub fn y(&self) -> f32 {
        self.0[1]
    }
    #[inline]
    pub fn z(&self) -> f32 {
        self.0[2]
    }

    #[inline]
    pub fn mut_x(&mut self) -> &mut f32 {
        &mut self.0[0]
    }
    #[inline]
    pub fn mut_y(&mut self) -> &mut f32 {
        &mut self.0[1]
    }
    #[inline]
    pub fn mut_z(&mut self) -> &mut f32 {
        &mut self.0[2]
    }

    #[inline]
    pub fn dot(mut self, other: Vec3) -> f32 {
        *self.mut_x() *= other.x();
        *self.mut_y() *= other.y();
        *self.mut_z() *= other.z();
        self.x() + self.y() + self.z()
    }

    #[inline]
    pub fn cross(self, other: Vec3) -> Vec3 {
        Vec3::new(
            self.y() * other.z() - self.z() * other.y(),
            self.z() * other.x() - self.x() * other.z(),
            self.x() * other.y() - self.y() * other.x(),
        )
    }
}

impl Default for Vec3 {
    fn default() -> Vec3 {
        Vec3([0., 0., 0.])
    }
}

impl Sum for Vec3 {
    fn sum<I>(iter: I) -> Self
    where
        I: Iterator<Item = Vec3>,
    {
        iter.fold(Vec3::default(), |acc, curr| acc + curr)
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

impl Add<f32> for Vec3 {
    type Output = Vec3;
    #[inline]
    fn add(mut self, other: f32) -> Vec3 {
        *self.mut_x() += other;
        *self.mut_y() += other;
        *self.mut_z() += other;
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

impl Mul<f32> for Vec3 {
    type Output = Vec3;
    #[inline]
    fn mul(mut self, other: f32) -> Vec3 {
        *self.mut_x() *= other;
        *self.mut_y() *= other;
        *self.mut_z() *= other;
        self
    }
}

impl Div for Vec3 {
    type Output = Vec3;
    #[inline]
    fn div(self, rhs: Vec3) -> Vec3 {
        Vec3::new(self.x() / rhs.x(), self.y() / rhs.y(), self.z() / rhs.z())
    }
}

impl Div<f32> for Vec3 {
    type Output = Vec3;
    #[inline]
    fn div(mut self, rhs: f32) -> Vec3 {
        *self.mut_x() /= rhs;
        *self.mut_y() /= rhs;
        *self.mut_z() /= rhs;
        self
    }
}

impl DivAssign for Vec3 {
    #[inline]
    fn div_assign(&mut self, rhs: Vec3) {
        *self.mut_x() /= rhs.x();
        *self.mut_y() /= rhs.y();
        *self.mut_z() /= rhs.z();
    }
}

impl Neg for Vec3 {
    type Output = Vec3;
    #[inline]
    fn neg(self) -> Vec3 {
        self * -1.
    }
}
impl From<(f32, f32, f32)> for Vec3 {
    fn from(triple: (f32, f32, f32)) -> Vec3 {
        Vec3::new(triple.0, triple.1, triple.2)
    }
}

impl From<f32> for Vec3 {
    fn from(val: f32) -> Vec3 {
        Vec3::new(val, val, val)
    }
}

impl<F> From<F> for Vec3
where
    F: FnMut() -> f32,
{
    fn from(mut f: F) -> Vec3 {
        Vec3::new(f(), f(), f())
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
