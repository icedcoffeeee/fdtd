use crate::*;

trait TFunc = FnOnce(f32) -> f32;

#[derive(Default)]
pub struct Source {
  freq: f32,
  center: Vec3,
  efield: (Vec3, f32),
  bfield: (Vec3, f32),
  t_func: Option<Box<dyn TFunc>>,
}

impl Source {
  pub fn with_freq(mut self, value: f32) -> Self {
    self.freq = value;
    self
  }
  pub fn with_center(mut self, value: Vec3) -> Self {
    self.center = value;
    self
  }
  pub fn with_efield(mut self, field: Vec3, phase: f32) -> Self {
    self.efield = (field, phase);
    self
  }
  pub fn with_bfield(mut self, field: Vec3, phase: f32) -> Self {
    self.bfield = (field, phase);
    self
  }

  pub fn gaussian(t0: f32, sigma: f32) -> Self {
    let mut source = Self::default();
    let gaussian = move |t: f32| (-sigma * (t - t0)).powi(2).exp();
    source.t_func = Some(Box::new(gaussian));
    source
  }
  pub fn smooth(t0: f32, sigma: f32) -> Self {
    let mut source = Self::default();
    let smooth = move |t: f32| 1. / (1. + (-sigma * (t - t0)).exp());
    source.t_func = Some(Box::new(smooth));
    source
  }
}
