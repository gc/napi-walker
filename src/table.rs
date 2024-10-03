use crate::counter::Counter;

#[derive(Clone)]
pub struct WalkerTable {
  aliases: Vec<usize>,
  probs: Vec<f32>,
}

impl WalkerTable {
  pub fn new(aliases: Vec<usize>, probs: Vec<f32>) -> WalkerTable {
    WalkerTable { aliases, probs }
  }

  pub fn next(&self, quantity: i32) -> Counter {
    let mut rng = fastrand::Rng::new();
    let mut counter = Counter::new();
    let len = self.probs.len() as i32;

    for _ in 0..quantity {
      counter.add(self.next_rng(&mut rng, len), 1)
    }
    counter
  }

  pub fn next_rng(&self, rng: &mut fastrand::Rng, len: i32) -> i32 {
    let i = rng.i32(0..len);
    let r = rng.f32();
    if r < self.probs[i as usize] {
      return self.aliases[i as usize] as i32;
    }
    i
  }
}
