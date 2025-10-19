use crate::*;

use std::f32::consts::TAU;

#[macro_export]
macro_rules! lerp {
    ($a:expr, $b:expr, $t:expr) => {
        $a * (1. - $t) + $b * $t
    };
}

pub trait Points {
  fn get_area(&self) -> f32;
  fn points_clockwise(&self) -> bool;
  fn point_is_inside(&self, point: &Vec3) -> bool;
}

impl Points for Vec<Vec3> {
  fn get_area(&self) -> f32 {
    let x: Vec<_> = self.iter().map(|p| p.x).collect();
    let mut y: Vec<_> = self.iter().map(|p| p.y).collect();

    y.rotate_left(1);
    let a: f32 = x.iter().zip(&y).map(|p| p.0 * p.1).sum();

    y.rotate_right(2);
    let b: f32 = x.iter().zip(&y).map(|p| p.0 * p.1).sum();

    a - b
  }

  fn points_clockwise(&self) -> bool {
    self.get_area() > 0.
  }

  fn point_is_inside(&self, point: &Vec3) -> bool {
    let angle: f32 = self
      .iter()
      .map_windows(|&[p1, p2]| (p1 - point).angle_between(p2 - point))
      .sum();

    (TAU - angle).abs() < TOL
  }
}
