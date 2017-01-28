extern crate num;

#[allow(dead_code)]
pub struct ZeroPaddedSignal<T> {
	values: Vec<T>,
}

impl<T: num::traits::Zero + Clone> ZeroPaddedSignal<T> {
	/**
		Returns the number of initialized values.
	*/
	#[allow(dead_code)]
	pub fn size(&self) -> usize {
		self.values.len()
	}

	/**
		Returns the value at the given index, zero if index
		is negative or uninitialized.
	*/
	#[allow(dead_code)]
	pub fn get(&self, idx: isize) -> T {
		/* Check for negative index: */
		if idx < 0 {
			/* Negative, return zero: */
			T::zero()
		} else {
			/* Not negative, cast to unsigned: */
			let idx = idx as usize;
			/* Check if the index is initialized: */
			if let Some(v) = self.values.get(idx).cloned() {
				/* It is, return the value: */
				v
			} else {
				/* It's not, return zero: */
				T::zero()
			}
		}
	}

	/**
		Returns a vector of signal values, starting with index start,
		ending with index end.
	*/
	#[allow(dead_code)]
	pub fn to_vector(&self, start: isize, end: isize) -> Vec<T> {
		/* Create an empty vector: */
		let mut x: Vec<T> = Vec::new();
		/* Loop through the given range: */
		for i in start..(end+1) {
			/* Add the values to the vector: */
			x.push(self.get(i));
		}
		/* Make the vector immutable: */
		let x = x;
		/* Return the vector: */
		x
	}
}

#[cfg(test)]
mod tests {
	use super::ZeroPaddedSignal;
	
	#[test]
	fn zero_padded_signal() {
		/* Create test signals: */
		let x1 = ZeroPaddedSignal { values: vec![42,7,11] };
		/* Test size method: */
		assert_eq!( 3, x1.size());
		/* Test get method: */
		assert_eq!( 0, x1.get(  -1));
		assert_eq!(42, x1.get(   0));
		assert_eq!( 7, x1.get(   1));
		assert_eq!(11, x1.get(   2));
		assert_eq!( 0, x1.get(   3));
		assert_eq!( 0, x1.get( 100));
		assert_eq!( 0, x1.get(-100));
		/* Test to_vector method: */
		assert_eq!(vec![0,0,0,42,7,11,0], x1.to_vector(-3,3));
	}
}
