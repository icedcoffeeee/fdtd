use crate::*;

#[derive(Default, Debug, Copy, Clone)]
pub struct Point {
  pub(crate) position: Vec3,
  pub(crate) e_field: Vec3,
  pub(crate) b_field: Vec3,
  pub(crate) epsilon: Complex32,
}

impl Point {
  pub fn new(x: f32, y: f32) -> Self {
    let mut p = Point::default();
    p.position.x = x;
    p.position.y = y;
    p
  }
}
