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

/**
  TODO
*/
#[allow(dead_code)]
struct MaximumLengthSequence<T> {
  coefficients: Vec<bool>,
  state: Vec<bool>,
  val_false: T,
  val_true: T
}

impl<T: num::traits::Num + Copy> MaximumLengthSequence<T> {
  /**
    Creates a new instance using the given coefficients
    and the given initial state.
  */
  #[allow(dead_code)]
  pub fn new(coefficients: Vec<bool>, state: Vec<bool>)
    -> MaximumLengthSequence<T> {
    /* Vectors must not be empty: */
    assert!(coefficients.len() > 0);
    assert!(state.len() > 0);
    /* Lengths of coefficients vector and state vector must match: */
    assert_eq!(coefficients.len(), state.len());
    /* Return the instance: */
    MaximumLengthSequence::<T> {
      coefficients: coefficients,
      state: state,
      val_false: T::zero(),
      val_true: T::one()
    }
  }
  
  /**
    Sets the two values the sequence can be.
  */
  #[allow(dead_code)]
  pub fn set_vals(&mut self, val_false: T, val_true: T) {
    self.val_false = val_false;
    self.val_true = val_true;
  }

  /**
    Returns the next value and switches to the next state.
  */
  #[allow(dead_code)]
  pub fn next(&mut self) -> T {
    /* Create the variable we need to return later: */
    let x: T;
    if self.state[self.state.len()-1] {
      x = self.val_true;
    } else {
      x = self.val_false;
    }
    /* Set the new state: */
    self.state = self.next_state(&self.state);
    /* Return the value: */
    x
  }
  
  /**
    Returns the next state for the given one.
  */
  fn next_state(&self, state: &Vec<bool>) -> Vec<bool> {
    /* Create a new state vector: */
    let mut new_state: Vec<bool> = Vec::new();
    /* Calculate the new values: */
    for i in 0..state.len() {
      /* TODO */
      new_state.push(false);
    }
    /* Make the new state immutable and return it: */
    let new_state = new_state;
    new_state
  }
}

#[cfg(test)]
mod tests {
  use super::ZeroPaddedSignal;
  use super::MaximumLengthSequence;
  
  #[test]
  fn zero_padded_signal() {
    /* Create test signals: */
    let mut x1: ZeroPaddedSignal<u32> = ZeroPaddedSignal {
      values: vec![42,7,11]
    };
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
    assert_eq!(0, x1.to_vector(3,-3).len());
    /* Test set method: */
    x1.set(0, 5);
    assert_eq!(vec![5,7,11], x1.values);
    x1.set(5, 12);
    assert_eq!(vec![5,7,11,0,0,12], x1.values);
    x1.set(6,100);
    assert_eq!(vec![5,7,11,0,0,12,100], x1.values);
  }
  
  #[test]
  fn linear_prediction() {
    /* Create test signals: */
    let x2: ZeroPaddedSignal<f64> = ZeroPaddedSignal {
      values: vec![1.,1.,1.,1.,1.,1.]
    };
    /* Test linear_prediction method: */
    assert_eq_floatvec!(
      vec![0.,0.8,0.8,0.8,0.7,0.7,0.7,-0.1,-0.1,-0.1],
      x2.linear_prediction(vec![0.8,0.,0.,-0.1]).values,
      1e-15
    );
  }

  #[test]
  fn maximum_length_sequence1() {
    let x1: MaximumLengthSequence<u8> =
      MaximumLengthSequence::new(vec![true,false,true],
        vec![true,true,true]);
  }
  
  #[test]
  #[should_panic(expected = "assertion failed: \
    coefficients.len() > 0")]
  fn maximum_length_sequence2() {
    let _: MaximumLengthSequence<u8> = MaximumLengthSequence::new(
      vec![], vec![true]);
  }

  #[test]
  #[should_panic(expected = "assertion failed: \
    state.len() > 0")]
  fn maximum_length_sequence3() {
    let _: MaximumLengthSequence<u8> = MaximumLengthSequence::new(
      vec![true], vec![]);
  }

  #[test]
  #[should_panic(expected = "assertion failed: \
    `(left == right)` (left: `2`, right: `1`)")]
  fn maximum_length_sequence4() {
    let _: MaximumLengthSequence<u8> = MaximumLengthSequence::new(
      vec![true,false], vec![true]);
  }
}
