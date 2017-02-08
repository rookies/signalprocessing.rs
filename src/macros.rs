/**
  Checks if two vectors with float values are
  approximately equal, i.e. they are the same
  length and the difference between each pair
  of values is below a given error.
*/
#[macro_export]
macro_rules! assert_eq_floatvec {
  ($ left: expr, $ right: expr, $ err: expr) => ({
    match (&($ left) , &($ right), &($ err)) {
      (left_val, right_val, err) => {
        /* Check vector lengths: */
        if ! (left_val.len() == right_val.len()) {
          panic!("assert_eq_floatvec failed: \
            `left.len() == right.len()` \
            (left.len(): `{:?}`, right.len(): `{:?}`)",
            left_val.len(), right_val.len())
        }
        /* Iterate over the values: */
        for i in 0..left_val.len() {
          /* Check value difference: */
          if ! ((left_val[i]-right_val[i]).abs() <= *err) {
            panic!("assert_eq_floatvec failed: \
              `abs(left-right) <= err` \
              (left: `{:?}`, right: `{:?}`, err: `{:?}`)",
              left_val[i], right_val[i], err)
          }
        }
      }
    }
  })
}

#[cfg(test)]
mod tests {
  #[test]
  fn assert_eq_floatvec_valid() {
    /* Create test vectors: */
    let x1: Vec<f64> = vec![1.,1.,1.];
    let x2: Vec<f64> = vec![1.,1.,1.];
    let x3: Vec<f64> = vec![1.1,1.,1.];
    /* These tests should be successful: */
    assert_eq_floatvec!(x1, x2, 0.);
    assert_eq_floatvec!(x1, x3, 0.11);
  }
  #[test]
  #[should_panic(expected = "assert_eq_floatvec failed")]
  fn assert_eq_floatvec_wrongsize1() {
    /* Create test vectors: */
    let x1: Vec<f64> = vec![1.,1.,1.];
    let x2: Vec<f64> = vec![1.,1.];
    /* This test should fail: */
    assert_eq_floatvec!(x1, x2, 100.);
  }
  #[test]
  #[should_panic(expected = "assert_eq_floatvec failed")]
  fn assert_eq_floatvec_wrongsize2() {
    /* Create test vectors: */
    let x1: Vec<f64> = vec![1.,1.,1.];
    let x2: Vec<f64> = vec![1.,1.,1.,1.];
    /* This test should fail: */
    assert_eq_floatvec!(x1, x2, 100.);
  }
  #[test]
  #[should_panic(expected = "assert_eq_floatvec failed")]
  fn assert_eq_floatvec_valdiff1() {
    /* Create test vectors: */
    let x1: Vec<f64> = vec![1.,1.,1.];
    let x2: Vec<f64> = vec![1.,1.2,1.];
    /* This test should fail: */
    assert_eq_floatvec!(x1, x2, 0.1);
  }
  #[test]
  #[should_panic(expected = "assert_eq_floatvec failed")]
  fn assert_eq_floatvec_valdiff2() {
    /* Create test vectors: */
    let x1: Vec<f64> = vec![1.,1.,1.];
    let x2: Vec<f64> = vec![1.1,1.,1.];
    /* This test should fail: */
    assert_eq_floatvec!(x1, x2, 0.);
  }
}
