use signals::periodic_signal::PeriodicSignal;

pub fn autocorrelation(sig: PeriodicSignal<f64>)
  -> PeriodicSignal<f64> {
  let mut vals: Vec<f64> = Vec::new();
  for k in 0..sig.size() {
    let mut val: f64 = 0.;
    for i in 0..sig.size() {
      val += sig.get(i as isize)*sig.get((i+k) as isize);
    }
    vals.push(val/(sig.size() as f64));
  }
  PeriodicSignal::new(vals)
}

#[cfg(test)]
mod tests {
  use super::*;
  use signals::periodic_signal::PeriodicSignal;
  use signals::maximum_length_sequence::MaximumLengthSequence;
  
  #[test]
  fn test1() {
    let mut x: MaximumLengthSequence<f64> = MaximumLengthSequence::new_predefined(3, vec![true;3]);
    x.set_vals(-0.58579f64,1f64);
    //assert_eq!(vec![1.], autocorrelation(PeriodicSignal::new(x.to_vector())).get_values());
  }
}
