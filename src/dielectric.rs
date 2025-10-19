use crate::*;

use std::f32::consts::TAU;

#[derive(Default)]
pub struct Dielectric {
  pub(crate) points: Vec<Vec3>,
  pub(crate) epsilon: Complex32,
}

impl Dielectric {
  pub fn from_points(mut points: Vec<Vec3>) -> Self {
    let mut dielectric = Self::default();

    if !points.points_clockwise() {
      points.reverse();
    }

    if points.last() != points.first() {
      points.push(*points.first().unwrap());
    }

    dielectric.points = points;
    dielectric
  }

  pub fn circle(center: Vec3, radius: f32) -> Self {
    let mut dielectric = Self::default();

    let res = 100;
    for alpha in 0..=res {
      let r = alpha as f32 / res as f32 * TAU;
      let p = Vec3::new(r.cos(), r.sin(), 0.) * radius + center;
      dielectric.points.push(p);
    }

    dielectric
  }

  pub fn with_epsilon(mut self, value: Complex32) -> Self {
    self.epsilon = value;
    self
  }
}
