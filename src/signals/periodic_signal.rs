extern crate num;

/**
  Models an infinite, periodic signal.
  Can be used with any type that implements
  num::traits::Num and Clone.
*/
#[allow(dead_code)]
pub struct PeriodicSignal<T> {
  values: Vec<T>,
}

impl<T: num::traits::Num + Clone> PeriodicSignal<T> {
  /**
    Creates a new signal using the given values.
  */
  #[allow(dead_code)]
  pub fn new(values: Vec<T>) -> PeriodicSignal<T> {
    PeriodicSignal::<T> {
      values: values
    }
  }
  
  /**
    Returns the number of initialized values,
    which is a period of the signal, but not
    necessarily the smallest.
  */
  #[allow(dead_code)]
  pub fn size(&self) -> usize {
    self.values.len()
  }
  
  /**
    Returns the value at the given index.
  */
  #[allow(dead_code)]
  pub fn get(&self, idx: isize) -> T {
    /* Move the index to a positive value and cast to unsigned: */
    let mut idx = idx;
    while idx < 0 {
      idx += self.values.len() as isize;
    }
    let mut idx = idx as usize;
    /* Move the index into the first period: */
    while idx >= self.values.len() {
      idx -= self.values.len();
    }
    /* Return the value: */
    if let Some(v) = self.values.get(idx).cloned() {
      v
    } else {
      T::zero()
    }
  }
  
  /**
    Returns a vector of signal values, starting with index start,
    ending with index end.
    TODO: Share implementation with ZeroPaddedSignal.
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
    Calculates the smallest period of the signal.
  */
  #[allow(dead_code)]
  pub fn period(&self) -> usize {
    /* Iterate over all possible periods: */
    for period in 1..self.size() {
      /* Check if this period is factor of the size: */
      if self.size() % period != 0 {
        continue;
      }
      /* Iterate over the offsets: */
      let mut offset: usize = period;
      let val = &self.values[0..period];
      let mut failed: bool = false;
      while offset <= self.size()-period {
        /* Check equality: */
        if ! (val == &self.values[offset..(offset+period)]) {
          failed = true;
          break;
        }
        /* Go to next offset: */
        offset += period;
      }
      /* If all values match, return the period: */
      if !failed {
        return period;
      }
    }
    /* No smaller period found, return size: */
    self.size()
  }
  
  /* TODO: Implement method. */
  /*
    Sets the signal value at the given index. If there's
    a gap between the last initialized index and the given
    one, the values between are initialized with zero.
  */
  /*#[allow(dead_code)]
  pub fn set(&mut self, idx: usize, val: T) {
    /* Initialize the gap values with zero, if necessary: */
    for _ in self.size()..(idx+1) {
      self.values.push(T::zero());
    }
    /* Set the new value: */
    self.values[idx] = val;
  }*/
}

#[cfg(test)]
mod tests {
  use super::PeriodicSignal;

  #[test]
  fn size() {
    /* Create test signals: */
    let x1: PeriodicSignal<u8> = PeriodicSignal::new(vec![1,1,1,1]);
    let x2: PeriodicSignal<f64> = PeriodicSignal::new(vec![1.]);
    /* Test `size` method: */
    assert_eq!(4, x1.size());
    assert_eq!(1, x2.size());
  }
  #[test]
  fn get() {
    /* Create test signal: */
    let x1: PeriodicSignal<u8> = PeriodicSignal::new(vec![1,2,3,4]);
    /* Test `get` method: */
    assert_eq!(2, x1.get(1));
    assert_eq!(4, x1.get(-1));
    assert_eq!(1, x1.get(4));
    assert_eq!(1, x1.get(400));
    assert_eq!(1, x1.get(-400));
  }
  #[test]
  fn to_vector() {
    /* Create test signal: */
    let x1: PeriodicSignal<u8> = PeriodicSignal::new(vec![1,2,3,4]);
    /* Test `to_vector` method: */
    assert_eq!(vec![1,2,3,4], x1.to_vector(0,3));
    assert_eq!(Vec::<u8>::new(), x1.to_vector(3,-3));
    assert_eq!(vec![1,2,3,4,1], x1.to_vector(0,4));
    assert_eq!(vec![4,1,2,3,4], x1.to_vector(-1,3));
  }
  #[test]
  fn period() {
    /* Create test signals: */
    let x1: PeriodicSignal<u8> = PeriodicSignal::new(vec![1,1,1,1]);
    let x2: PeriodicSignal<u8> = PeriodicSignal::new(vec![1,0,1,0]);
    let x3: PeriodicSignal<u8> = PeriodicSignal::new(vec![1,0,1,1]);
    /* Check `period` method: */
    assert_eq!(1, x1.period());
    assert_eq!(2, x2.period());
    assert_eq!(4, x3.period());
  }
}
