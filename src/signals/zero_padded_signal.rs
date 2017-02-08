extern crate num;

/**
  Models an infinite signal, negative indices return
  always zero (to ensure causality of the signal),
  uninitialized positive indices, too.
  Can be used with any type that implements
  num::traits::Num and Clone.
*/
#[allow(dead_code)]
pub struct ZeroPaddedSignal<T> {
  values: Vec<T>,
}

impl<T: num::traits::Num + Clone> ZeroPaddedSignal<T> {
  /**
    Creates a new signal using the given values.
  */
  #[allow(dead_code)]
  pub fn new(values: Vec<T>) -> ZeroPaddedSignal<T> {
    ZeroPaddedSignal::<T> {
      values: values
    }
  }
  
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
  
  /**
    Creates a new signal by doing Linear Prediction using
    the given coefficients.
  */
  #[allow(dead_code)]
  pub fn linear_prediction(&self, a: Vec<T>) -> ZeroPaddedSignal<T> {
    /* Create a vector for the values: */
    let mut vals: Vec<T> = Vec::new();
    /* Calculate the size of the new signal: */
    let size: usize = self.size() + a.len();
    /* Iterate over the new indices: */
    for i in 0..size {
      let mut val: T = T::zero();
      /* ... and the coefficients: */
      for j in 0..a.len() {
        /* Calculate and typecast the index in the old signal: */
        let idx = (i as isize) - ((j+1) as isize);
        /* Weight the value and add it up: */
        if let Some(v) = a.get(j).cloned() {
          val = val + v*self.get(idx);
        }
      }
      /* Add the value to the vector: */
      vals.push(val);
    }
    /* Create a signal from the vector and return it: */
    let x: ZeroPaddedSignal<T> = ZeroPaddedSignal { values: vals };
    x
  }

  /**
    Sets the signal value at the given index. If there's
    a gap between the last initialized index and the given
    one, the values between are initialized with zero.
  */
  #[allow(dead_code)]
  pub fn set(&mut self, idx: usize, val: T) {
    /* Initialize the gap values with zero, if necessary: */
    for _ in self.size()..(idx+1) {
      self.values.push(T::zero());
    }
    /* Set the new value: */
    self.values[idx] = val;
  }
}

#[cfg(test)]
mod tests {
  use super::ZeroPaddedSignal;
  
  #[test]
  fn zero_padded_signal_size() {
    /* Create test signal: */
    let x1: ZeroPaddedSignal<u32> =
      ZeroPaddedSignal::new(vec![42,7,11]);
    /* Test `size` method: */
    assert_eq!( 3, x1.size());
  }
  #[test]
  fn zero_padded_signal_get() {
    /* Create test signal: */
    let x1: ZeroPaddedSignal<u32> =
      ZeroPaddedSignal::new(vec![42,7,11]);
    /* Test `get` method: */
    assert_eq!( 0, x1.get(  -1));
    assert_eq!(42, x1.get(   0));
    assert_eq!( 7, x1.get(   1));
    assert_eq!(11, x1.get(   2));
    assert_eq!( 0, x1.get(   3));
    assert_eq!( 0, x1.get( 100));
    assert_eq!( 0, x1.get(-100));
  }
  #[test]
  fn zero_padded_signal_to_vector() {
    /* Create test signal: */
    let x1: ZeroPaddedSignal<u32> =
      ZeroPaddedSignal::new(vec![42,7,11]);
    /* Test `to_vector` method: */
    assert_eq!(vec![0,0,0,42,7,11,0], x1.to_vector(-3,3));
    assert_eq!(0, x1.to_vector(3,-3).len());
  }
  #[test]
  fn zero_padded_signal_set() {
    /* Create test signal: */
    let mut x1: ZeroPaddedSignal<u32> =
      ZeroPaddedSignal::new(vec![42,7,11]);
    /* Test `set` method: */
    x1.set(0, 5);
    assert_eq!(vec![5,7,11], x1.values);
    x1.set(5, 12);
    assert_eq!(vec![5,7,11,0,0,12], x1.values);
    x1.set(6,100);
    assert_eq!(vec![5,7,11,0,0,12,100], x1.values);
  }
  #[test]
  fn zero_padded_signal_linear_prediction() {
    /* Create test signal: */
    let x1: ZeroPaddedSignal<f64> =
      ZeroPaddedSignal::new(vec![1.,1.,1.,1.,1.,1.]);
    /* Test `linear_prediction` method: */
    assert_eq_floatvec!(
      vec![0.,0.8,0.8,0.8,0.7,0.7,0.7,-0.1,-0.1,-0.1],
      x1.linear_prediction(vec![0.8,0.,0.,-0.1]).values,
      1e-15
    );
  }
}
