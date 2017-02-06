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
	fn assert_eq_floatvec1() {
		/* Create test vectors: */
		let x1: Vec<f64> = vec![1.,1.,1.];
		let x2: Vec<f64> = vec![1.,1.,1.];
		let x3: Vec<f64> = vec![1.1,1.,1.];
		/* These tests should be successful: */
		assert_eq_floatvec!(x1, x2, 0.);
		assert_eq_floatvec!(x1, x3, 0.11);
	}

	#[test]
	#[should_panic]
	fn assert_eq_floatvec2() {
		/* Create test vectors: */
		let x1: Vec<f64> = vec![1.,1.,1.];
		let x3: Vec<f64> = vec![1.1,1.,1.];
		let x4: Vec<f64> = vec![1.,1.2,1.];
		let x5: Vec<f64> = vec![1.,1.];
		let x6: Vec<f64> = vec![1.,1.,1.,1.];
		/* These tests should fail: */
		assert_eq_floatvec!(x1, x5, 100.);
		assert_eq_floatvec!(x1, x6, 100.);
		assert_eq_floatvec!(x1, x4, 0.1);
		assert_eq_floatvec!(x1, x3, 0.);
	}
}
