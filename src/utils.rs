use rayon::slice::ParallelSlice;

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
    let x: Vec<_> = self.par_iter().map(|p| p.x).collect();
    let mut y: Vec<_> = self.par_iter().map(|p| p.y).collect();

    y.rotate_left(1);
    let a: f32 = x.par_iter().zip(&y).map(|p| p.0 * p.1).sum();

    y.rotate_right(2);
    let b: f32 = x.par_iter().zip(&y).map(|p| p.0 * p.1).sum();

    a - b
  }

  fn points_clockwise(&self) -> bool {
    self.get_area() > 0.
  }

  fn point_is_inside(&self, point: &Vec3) -> bool {
    let angle: f32 = self
      .par_windows(2)
      .map(|p| (p[0] - point).angle_between(p[1] - point))
      .sum();

    (TAU - angle).abs() < TOL
  }
}
