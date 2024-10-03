use napi::bindgen_prelude::Int32Array;
use std::collections::HashMap;

pub struct Counter {
  counts: HashMap<i32, i32>,
}

impl Counter {
  pub fn new() -> Self {
    Self {
      counts: HashMap::new(),
    }
  }

  pub fn add(&mut self, id: i32, amount: i32) {
    *self.counts.entry(id).or_insert(0) += amount;
  }

  pub fn to_json(self) -> String {
    serde_json::to_string(&self.counts).unwrap()
  }

  pub fn to_js(&self) -> Int32Array {
    Int32Array::new(self.counts.values().cloned().collect::<Vec<_>>())
  }

  pub fn add_counter(&mut self, other: &Counter) {
    for (&id, &count) in &other.counts {
      *self.counts.entry(id).or_insert(0) += count;
    }
  }
}
