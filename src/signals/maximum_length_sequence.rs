extern crate num;

/**
  Models a maximum length sequence generator.
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
    Example polynom: p(x) = x^4 + a3*x^3 + a2*x^2 + a1*x + 1
      leads to coefficients: [a1,a2,a3]
  */
  #[allow(dead_code)]
  pub fn new(coefficients: Vec<bool>, state: Vec<bool>)
    -> MaximumLengthSequence<T> {
    /* Vectors must not be empty: */
    assert!(state.len() > 0);
    /* Lengths of coefficients vector and state vector must match: */
    assert_eq!(coefficients.len()+1, state.len());
    /* Return the instance: */
    MaximumLengthSequence::<T> {
      coefficients: coefficients,
      state: state,
      val_false: T::zero(),
      val_true: T::one()
    }
  }
  
  /**
    Creates a new instance using a pre-defined polynom
    and the given initial state.
    Used polynoms:
      p(x) = x + 1
      p(x) = x^2 + x + 1
      p(x) = x^3 + x + 1
      p(x) = x^4 + x + 1
      p(x) = x^5 + x^2 + 1
      p(x) = x^6 + x + 1
      p(x) = x^7 + x + 1
      p(x) = x^8 + x^6 + x^5 + x + 1
      p(x) = x^9 + x^4 + 1
      p(x) = x^10 + x^3 + 1
  */
  #[allow(dead_code)]
  pub fn new_predefined(order: u8, state: Vec<bool>)
    -> MaximumLengthSequence<T> {
    match order {
      1 => MaximumLengthSequence::<T>::new(vec![], state),
      2 => MaximumLengthSequence::<T>::new(vec![true], state),
      3 => MaximumLengthSequence::<T>::new(vec![true,false], state),
      4 => MaximumLengthSequence::<T>::new(
        vec![true,false,false], state),
      5 => MaximumLengthSequence::<T>::new(
        vec![false,true,false,false], state),
      6 => MaximumLengthSequence::<T>::new(
        vec![true,false,false,false,false], state),
      7 => MaximumLengthSequence::<T>::new(
        vec![true,false,false,false,false,false], state),
      8 => MaximumLengthSequence::<T>::new(
        vec![true,false,false,false,true,true,false], state),
      9 => MaximumLengthSequence::<T>::new(
        vec![false,false,false,true,false,false,false,false], state),
      10 => MaximumLengthSequence::<T>::new(
        vec![false,false,true,false,false,false,false,false,false],
        state),
      _ => panic!("Sorry, no polynom for order {}, yet.", order)
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
    self.state = MaximumLengthSequence::<T>::next_state(&self.state,
      &self.coefficients);
    /* Return the value: */
    x
  }

  /**
    Returns one period as a vector, does not change
    the internal state.
  */
  #[allow(dead_code)]
  pub fn to_vector(self) -> Vec<T> {
    /* Create a temporary internal state */
    let mut state: Vec<bool> = self.state;
    /* Create a vector that we return later: */
    let mut x: Vec<T> = Vec::new();
    /* Iterate over the period: */
    for _ in 0..(2usize.pow(state.len() as u32)-1) {
      /* Add the value: */
      if state[state.len()-1] {
        x.push(self.val_true);
      } else {
        x.push(self.val_false);
      }
      /* And set the new state: */
      state = MaximumLengthSequence::<T>::next_state(&state,
        &self.coefficients);
    }
    /* Make the vector immutable and return it: */
    let x = x;
    x
  }
  
  /**
    Returns the next state for the given one.
  */
  fn next_state(state: &Vec<bool>, coefficients: &Vec<bool>)
    -> Vec<bool> {
    /* Create a new state vector: */
    let mut new_state: Vec<bool> = Vec::new();
    /* Create the new first value: */
    let mut first: bool = state[state.len()-1];
    for i in 0..(state.len()-1) {
      if coefficients[i] {
        first ^= state[state.len()-(i+2)];
      }
    }
    new_state.push(first);
    /* Shift the other values through: */
    for i in 1..state.len() {
      new_state.push(state[i-1]);
    }
    /* Make the new state immutable and return it: */
    let new_state = new_state;
    new_state
  }
}

#[cfg(test)]
mod tests {
  use super::MaximumLengthSequence;
  use signals::periodic_signal::PeriodicSignal;

  #[test]
  fn maximum_length_sequence_next() {
    /* Create test sequences: */
    /* x^3 + x + 1; init state: 0-1-1 */
    let mut x1: MaximumLengthSequence<u8> =
      MaximumLengthSequence::new(vec![true,false],
        vec![false,true,true]);
     /* x^3 + x + 1; init state: 1-0-0 */
     let mut x2: MaximumLengthSequence<u8> =
      MaximumLengthSequence::new(vec![true,false],
        vec![true,false,false]);
    /* Test `next` method. */
    for v in vec![1,1,0,0,1,0,1] {
      assert_eq!(v, x1.next());
    }
    for v in vec![0,0,1,0,1,1,1] {
      assert_eq!(v, x2.next());
    }
  }
  #[test]
  fn maximum_length_sequence_to_vector() {
    /* Create test sequences: */
    /* x^3 + x + 1; init state: 0-1-1 */
    let x1: MaximumLengthSequence<u8> =
      MaximumLengthSequence::new(vec![true,false],
        vec![false,true,true]);
     /* x^3 + x + 1; init state: 1-0-0 */
     let x2: MaximumLengthSequence<u8> =
      MaximumLengthSequence::new(vec![true,false],
        vec![true,false,false]);
    /* Test `to_vector` method: */
    assert_eq!(vec![1,1,0,0,1,0,1], x1.to_vector());
    assert_eq!(vec![0,0,1,0,1,1,1], x2.to_vector());
  }
  #[test]
  #[should_panic(expected = "assertion failed: \
    state.len() > 0")]
  fn maximum_length_sequence_new1() {
    let _: MaximumLengthSequence<u8> = MaximumLengthSequence::new(
      vec![true], vec![]);
  }
  #[test]
  #[should_panic(expected = "assertion failed: \
    `(left == right)` (left: `3`, right: `1`)")]
  fn maximum_length_sequence_new2() {
    let _: MaximumLengthSequence<u8> = MaximumLengthSequence::new(
      vec![true,false], vec![true]);
  }
  #[test]
  fn maximum_length_sequence_set_vals() {
    /* Create test sequence: */
    /* x^3 + x + 1; init state: 0-1-1 */
    let mut x1: MaximumLengthSequence<i8> =
      MaximumLengthSequence::new(vec![true,false],
        vec![false,true,true]);
    /* Test `set_vals` method: */
    x1.set_vals(-5,5);
    assert_eq!(vec![5,5,-5,-5,5,-5,5], x1.to_vector());
  }
  #[test]
  fn maximum_length_sequence_new_predefined() {
    /* Create test results: */
    let vals = vec![
      vec![1],
      vec![1,1,0],
      vec![1,1,1,0,0,1,0],
      vec![1,1,1,1,0,0,0,1,0,0,1,1,0,1,0],
      vec![1,1,1,1,1,0,0,0,1,1,0,1,1,1,0,1,0,1,0,0,0,0,1,0,0,1,0,1,1,
        0,0],
      vec![1,1,1,1,1,1,0,0,0,0,0,1,0,0,0,0,1,1,0,0,0,1,0,1,0,0,1,1,1,
        1,0,1,0,0,0,1,1,1,0,0,1,0,0,1,0,1,1,0,1,1,1,0,1,1,0,0,1,1,0,
        1,0,1,0],
      vec![1,1,1,1,1,1,1,0,0,0,0,0,0,1,0,0,0,0,0,1,1,0,0,0,0,1,0,1,0,
        0,0,1,1,1,1,0,0,1,0,0,0,1,0,1,1,0,0,1,1,1,0,1,0,1,0,0,1,1,1,
        1,1,0,1,0,0,0,0,1,1,1,0,0,0,1,0,0,1,0,0,1,1,0,1,1,0,1,0,1,1,
        0,1,1,1,1,0,1,1,0,0,0,1,1,0,1,0,0,1,0,1,1,1,0,1,1,1,0,0,1,1,
        0,0,1,0,1,0,1,0],
      vec![1,1,1,1,1,1,1,1,0,0,1,0,1,1,1,1,0,1,0,0,1,0,1,0,0,0,0,1,1,
        0,1,1,1,0,1,1,0,1,1,1,1,1,0,1,0,1,1,1,0,1,0,0,0,0,0,1,1,0,0,
        1,0,1,0,1,0,1,0,0,0,1,1,0,1,0,1,1,0,0,0,1,1,0,0,0,0,0,1,0,0,
        1,0,1,1,0,1,1,0,1,0,1,0,0,1,1,0,1,0,0,1,1,1,1,1,1,0,1,1,1,0,
        0,1,1,0,0,1,1,1,1,0,1,1,0,0,1,0,0,0,0,1,0,0,0,0,0,0,1,1,1,0,
        0,1,0,0,1,0,0,1,1,0,0,0,1,0,0,1,1,1,0,1,0,1,0,1,1,0,1,0,0,0,
        1,0,0,0,1,0,1,0,0,1,0,0,0,1,1,1,1,1,0,0,0,0,0,0,0,1,0,1,1,1,
        0,0,0,1,1,1,0,1,1,1,1,0,0,0,1,0,1,1,0,0,1,1,0,1,1,0,0,0,0,1,
        1,1,1,0,0,1,1,1,0,0,0,0,1,0,1,0],
      vec![1,1,1,1,1,1,1,1,1,0,0,0,0,0,1,1,1,1,0,1,1,1,1,1,0,0,0,1,0,
        1,1,1,0,0,1,1,0,0,1,0,0,0,0,0,1,0,0,1,0,1,0,0,1,1,1,0,1,1,0,
        1,0,0,0,1,1,1,1,0,0,1,1,1,1,1,0,0,1,1,0,1,1,0,0,0,1,0,1,0,1,
        0,0,1,0,0,0,1,1,1,0,0,0,1,1,0,1,1,0,1,0,1,0,1,1,1,0,0,0,1,0,
        0,1,1,0,0,0,1,0,0,0,1,0,0,0,0,0,0,0,0,1,0,0,0,0,1,0,0,0,1,1,
        0,0,0,0,1,0,0,1,1,1,0,0,1,0,1,0,1,0,1,1,0,0,0,0,1,1,0,1,1,1,
        1,0,1,0,0,1,1,0,1,1,1,0,0,1,0,0,0,1,0,1,0,0,0,0,1,0,1,0,1,1,
        0,1,0,0,1,1,1,1,1,1,0,1,1,0,0,1,0,0,1,0,0,1,0,1,1,0,1,1,1,1,
        1,1,0,0,1,0,0,1,1,0,1,0,1,0,0,1,1,0,0,1,1,0,0,0,0,0,0,0,1,1,
        0,0,0,1,1,0,0,1,0,1,0,0,0,1,1,0,1,0,0,1,0,1,1,1,1,1,1,1,0,1,
        0,0,0,1,0,1,1,0,0,0,1,1,1,0,1,0,1,1,0,0,1,0,1,1,0,0,1,1,1,1,
        0,0,0,1,1,1,1,1,0,1,1,1,0,1,0,0,0,0,0,1,1,0,1,0,1,1,0,1,1,0,
        1,1,1,0,1,1,0,0,0,0,0,1,0,1,1,0,1,0,1,1,1,1,1,0,1,0,1,0,1,0,
        1,0,0,0,0,0,0,1,0,1,0,0,1,0,1,0,1,1,1,1,0,0,1,0,1,1,1,0,1,1,
        1,0,0,0,0,0,0,1,1,1,0,0,1,1,1,0,1,0,0,1,0,0,1,1,1,1,0,1,0,1,
        1,1,0,1,0,1,0,0,0,1,0,0,1,0,0,0,0,1,1,0,0,1,1,1,0,0,0,0,1,0,
        1,1,1,1,0,1,1,0,1,1,0,0,1,1,0,1,0,0,0,0,1,1,1,0,1,1,1,1,0,0,
        0,0],
      vec![1,1,1,1,1,1,1,1,1,1,0,0,0,0,0,0,0,1,1,1,0,0,0,0,1,1,1,1,1,
        1,0,1,1,1,0,0,0,1,0,0,1,1,1,1,1,0,0,0,1,1,0,0,1,1,1,1,1,0,1,
        0,1,1,0,0,1,0,1,1,0,0,1,0,0,1,0,0,1,0,0,0,0,0,0,0,0,0,1,0,0,
        0,0,0,0,1,0,0,1,0,0,0,1,0,0,0,0,0,1,1,0,0,1,0,0,1,1,0,1,0,0,
        0,0,1,0,0,1,0,1,0,1,0,0,0,0,1,1,1,1,0,1,0,1,1,1,0,1,0,1,1,0,
        1,1,0,1,1,0,0,0,0,0,0,0,0,1,1,0,0,0,0,0,1,1,0,1,1,0,0,1,1,0,
        0,0,0,1,0,1,0,1,1,0,1,0,1,1,1,0,0,0,1,1,0,1,1,1,1,1,1,0,0,0,
        1,0,0,0,1,1,1,1,0,0,1,1,1,1,0,1,1,0,1,1,0,1,0,0,0,0,0,0,0,1,
        0,1,0,0,0,0,1,0,1,1,0,1,0,1,0,1,0,0,0,1,1,1,1,1,0,1,1,1,1,0,
        0,1,0,0,1,0,1,1,0,0,0,0,0,1,0,0,1,1,0,0,1,0,0,0,1,0,1,0,0,0,
        1,1,0,1,1,0,1,1,1,0,0,0,0,0,0,1,1,1,1,0,0,0,1,1,1,0,1,1,1,1,
        1,1,1,0,0,1,0,0,0,0,1,1,0,0,0,1,0,1,1,0,1,1,1,0,1,0,0,0,0,1,
        1,0,1,0,1,0,1,1,0,0,1,1,1,1,0,0,1,0,1,1,0,1,1,0,0,1,0,0,0,0,
        0,1,0,0,0,1,0,0,1,0,0,1,1,0,0,0,0,0,0,1,0,1,1,0,0,0,1,0,1,0,
        0,1,1,1,0,1,1,0,0,1,1,1,0,0,0,1,0,1,1,1,1,1,1,0,1,0,1,0,0,0,
        1,0,1,1,1,0,1,1,0,1,0,1,1,0,0,0,0,1,1,0,0,1,1,0,1,1,0,1,0,1,
        0,0,0,0,0,1,1,1,0,1,0,0,1,1,1,1,0,1,0,0,1,1,0,1,0,1,0,0,1,0,
        0,1,1,1,0,0,0,0,0,1,1,1,1,1,0,0,1,1,1,0,0,1,1,0,1,1,1,1,0,1,
        0,0,0,1,0,1,0,1,0,1,1,0,1,1,1,1,1,0,0,0,0,1,0,0,1,1,1,0,1,0,
        0,0,1,1,1,0,1,0,1,1,1,1,1,0,1,1,0,1,0,0,1,0,0,0,0,1,0,0,0,0,
        1,0,1,0,0,1,0,1,0,1,1,0,0,0,1,1,1,0,0,1,1,1,1,1,1,1,0,1,1,0,
        0,0,0,1,0,0,0,1,1,0,1,0,0,1,1,1,0,0,1,0,0,1,1,1,1,0,0,0,0,1,
        1,0,1,1,1,0,1,1,0,0,0,1,1,0,0,0,1,1,1,1,0,1,1,1,1,1,0,1,0,0,
        1,0,0,1,0,1,0,0,0,0,0,0,1,1,0,1,0,0,0,1,1,0,0,1,0,1,1,1,0,1,
        0,0,1,0,1,1,0,1,0,0,0,1,0,0,0,1,0,1,1,0,0,1,1,0,1,0,0,1,0,1,
        0,0,1,0,0,0,1,1,0,0,0,0,1,1,1,0,1,1,0,1,1,1,1,0,0,0,0,0,1,0,
        1,1,1,0,0,1,0,1,0,1,1,1,0,0,1,1,1,0,1,1,1,0,1,1,1,0,0,1,1,0,
        0,1,1,1,0,1,0,1,0,1,1,1,0,1,1,1,1,0,1,1,0,0,1,0,1,0,0,0,1,0,
        0,1,1,0,1,1,0,0,0,1,0,0,0,0,1,1,1,0,0,1,0,1,1,1,1,1,0,0,1,0,
        1,0,0,1,1,0,0,1,1,0,0,1,0,1,0,1,0,1,0,0,1,1,1,1,1,1,0,0,1,1,
        0,0,0,1,1,0,1,0,1,1,1,1,0,0,1,1,0,1,0,1,1,0,1,0,0,1,1,0,0,0,
        1,0,0,1,0,1,1,1,0,0,0,0,1,0,1,1,1,1,0,1,0,1,0,1,0,1,0,1,1,1,
        1,1,1,1,1,0,1,0,0,0,0,0,1,0,1,0,1,0,0,1,0,1,1,1,1,0,0,0,1,0,
        1,0,1,1,1,1,0,1,1,1,0,1,0,1,0,0,1,1,0,1,1,1,0,0,1,0,0,0,1,1,
        1,0,0,0]
    ];
    /* Test `new_predefined` method: */
    for i in 0..vals.len() {
      /* Equal values: */
      let x: MaximumLengthSequence<u8> =
        MaximumLengthSequence::new_predefined(
          (i+1) as u8, vec![true;i+1]);
      let v: Vec<u8> = x.to_vector();
      assert_eq!(vals[i], v);
      /* Correct period: */
      let y: PeriodicSignal<u8> = PeriodicSignal::new(v);
      assert_eq!((2u32.pow((i+1) as u32) as usize)-1, y.period());
    }
  }
}
