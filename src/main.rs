use fdtd::*;

fn main() {
  // define bounds of dielectrics
  let circle = Dielectric::circle(Vec3::ZERO, 5.).with_epsilon(Complex::new(1.33, 0.));

  // define sources
  let source = Source::gaussian(0., 0.2)
    .with_freq(1e9)
    .with_center(Vec3::new(-18., 18., 0.))
    .with_efield(Vec3::new(0., 1., 0.), 0.);

  // define space and time ranges
  let mut field = Field::new((-20., 20., 10), (-20., 20., 10), (1., 10))
    .add_dielectric(circle)
    .add_source(source);

  // run and save to file
  field.run(Some("./test.mp4"));

  // query field values at any point
  println!("{at:#?}", at = field.at(0., 0.))
}
