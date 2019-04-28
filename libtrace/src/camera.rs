use crate::{Ray, Vec3};
use std::f32;
use rand::Rng;

#[derive(Debug, PartialEq, Clone)]
pub struct Camera {
  origin: Vec3,
  lower_left_corner: Vec3,
  horizontal: Vec3,
  vertical: Vec3,
  lens_radius: f32,

  shutter_open_time: f32,
  shutter_close_time: f32,

  u: Vec3,
  v: Vec3,
  w: Vec3,
}

impl Camera {
  pub fn new(
    lookfrom: Vec3,
    lookat: Vec3,
    view_up: Vec3,
    vfov: f32,
    aspect: f32,
    aperture: f32,
    focus_dist: f32,
    shutter_open_time: f32,
    shutter_close_time: f32,
  ) -> Camera {
    let lens_radius = aperture / 2.;
    let theta = vfov * f32::consts::PI / 180.;
    let half_height = (theta / 2.).tan();
    let half_width = aspect * half_height;

    let origin = lookfrom;

    let w = (lookfrom - lookat).into_normalized();
    let u = view_up.cross(&w).into_normalized();
    let v = w.cross(&u);

    let lower_left_corner =
      origin - u * half_width * focus_dist - v * half_height * focus_dist - w * focus_dist;

    let horizontal = u * 2. * half_width * focus_dist;
    let vertical = v * 2. * half_height * focus_dist;

    Camera {
      shutter_close_time,
      shutter_open_time,
      lens_radius,
      origin,
      lower_left_corner,
      horizontal,
      vertical,
      u,
      v,
      w,
    }
  }

  #[inline]
  pub fn get_ray(&self, s: f32, t: f32) -> Ray {
    let rd = Vec3::random_in_unit_circle() * self.lens_radius;
    let offset = self.u * rd.x() + self.v * rd.y();
    let time = rand::thread_rng().gen_range(self.shutter_open_time, self.shutter_close_time);


    Ray::new(
      self.origin + offset,
      self.lower_left_corner + self.horizontal * s + self.vertical * t - self.origin - offset,
      time
    )
  }
}
