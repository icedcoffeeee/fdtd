use crate::*;

use std::path::Path;

/// (start, end, unit density)
type Range = (f32, f32, i32);
/// (end, unit density)
type TRange = (f32, i32);

pub struct Field {
  pub(crate) t_range: TRange,
  pub(crate) field: Vec<Vec<Point>>,
  pub(crate) sources: Vec<Source>,
}

impl Field {
  pub fn new(x_range: Range, y_range: Range, t_range: TRange) -> Self {
    let field = Field::make_field(x_range, y_range);
    let sources = Vec::new();

    Self { t_range, field, sources }
  }

  // This is where we read the points enclosed by the shape and set the field point values to the
  // set dielectric amount. Suuuper slow, checks each point if inside the region.
  pub fn add_dielectric(mut self, dielectric: Dielectric) -> Self {
    self.field.par_iter_mut().for_each(|ps| {
      ps.par_iter_mut()
        .filter(|p| dielectric.points.point_is_inside(&p.position))
        .for_each(|p| {
          if p.epsilon.abs() == 0. {
            p.epsilon = dielectric.epsilon
          } else {
            p.epsilon = lerp!(p.epsilon, dielectric.epsilon, 0.5);
          }
        })
    });

    self
  }

  // This is accumulated for the run function.
  pub fn add_source(mut self, source: Source) -> Self {
    self.sources.push(source);
    self
  }

  pub fn run<P: AsRef<Path>>(&mut self, save: Option<P>) {}

  /// Returns the nearest field value on the grid
  pub fn at(&self, x: f32, y: f32) -> Point {
    *self
      .field
      .par_iter()
      .map(|ps| ps.par_iter().find(|p| p.position.y > y))
      .filter(|ps| ps.is_some())
      .map(|ps| ps.unwrap())
      .find(|ps| ps.position.x > x)
      .expect("point not in field")
  }

  fn make_field(x_range: Range, y_range: Range) -> Vec<Vec<Point>> {
    let (x0, x1, rx) = x_range;
    let (y0, y1, ry) = y_range;
    let nx = (x1 - x0) * rx as f32;
    let ny = (y1 - y0) * ry as f32;

    (0..nx as usize)
      .map(|i| i as f32 / nx)
      .map(|ix| {
        (0..ny as usize)
          .map(|i| i as f32 / ny)
          .map(|iy| {
            let x = lerp!(x0, x1, ix);
            let y = lerp!(y0, y1, iy);
            Point::new(x, y)
          })
          .collect()
      })
      .collect()
  }
}
