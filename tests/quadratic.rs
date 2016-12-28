extern crate dualnum;
extern crate rootfinding;

use dualnum::*;
use rootfinding::{bisection_root};

#[test]
fn quadratic_test() {
  fn q(x: DualNum<f32>) -> DualNum<f32> {
    (x - 1.0) * (x - 4.0)
  }

  let eps = 1.0e-6;
  let r1 = bisection_root(0.0, 2.0, eps, |x| {
    let x = DualNum::param(x);
    q(x).real()
  }).unwrap();
  let r2 = bisection_root(3.0, 5.0, eps, |x| {
    let x = DualNum::param(x);
    q(x).real()
  }).unwrap();
  let m = bisection_root(0.0, 5.0, eps, |x| {
    let x = DualNum::param(x);
    q(x).dual()
  }).unwrap();

  assert!((r1 - 1.0).abs() <= eps);
  assert!((r2 - 4.0).abs() <= eps);
  assert!((m - 2.5).abs() <= eps);
}
