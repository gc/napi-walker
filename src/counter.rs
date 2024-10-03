use napi::bindgen_prelude::Int32Array;

pub struct Counter {
  counts: Vec<i32>,
}

impl Counter {
  pub fn new() -> Self {
    Self {
      counts: vec![0; 2222 as usize + 1],
    }
  }

  pub fn add(&mut self, id: i32, amount: i32) {
    self.counts[id as usize] += amount;
  }

  pub fn to_js(&self) -> Int32Array {
    Int32Array::new(self.counts.clone())
  }

  pub fn add_counter(&mut self, other: &Counter) {
    for (i, count) in other.counts.iter().enumerate() {
      self.counts[i] += count;
    }
  }
}
