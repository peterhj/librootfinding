extern crate arithmetic;

use arithmetic::*;

pub enum RootError {
  BadBracket,
}

pub type RootResult<T> = Result<T, RootError>;

pub fn bisection_root<T, F>(mut x_lo: T, mut x_hi: T, eps: T, mut func: F) -> RootResult<T> where T: Copy + PartialOrd + Field, F: FnMut(T) -> T {
  let mut y_lo = func(x_lo);
  let mut y_hi = func(x_hi);
  if y_lo * y_hi > T::zero() {
    return Err(RootError::BadBracket);
  } else if y_lo == T::zero() {
    return Ok(x_lo);
  } else if y_hi == T::zero() {
    return Ok(x_hi);
  }
  let half = T::one() / (T::one() + T::one());
  loop {
    let x_err = half * (x_hi - x_lo);
    let x_mid = half * (x_hi + x_lo);
    if x_err <= eps {
      return Ok(x_mid);
    }
    let y_mid = func(x_mid);
    if y_lo * y_mid < T::zero() {
      x_hi = x_mid;
      y_hi = y_mid;
    } else if y_mid * y_hi < T::zero() {
      x_lo = x_mid;
      y_lo = y_mid;
    } else if y_mid == T::zero() {
      return Ok(x_mid);
    } else {
      unreachable!();
    }
  }
}
