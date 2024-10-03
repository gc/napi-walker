use rustc_hash::FxHashMap;

pub struct Counter {
  counts: FxHashMap<i32, i32>,
}

impl Default for Counter {
  fn default() -> Self {
    Self::new()
  }
}

impl Counter {
  pub fn new() -> Self {
    Self {
      counts: FxHashMap::default(),
    }
  }

  pub fn add(&mut self, id: i32, amount: i32) {
    *self.counts.entry(id).or_insert(0) += amount;
  }

  pub fn to_json(self) -> String {
    serde_json::to_string(&self.counts).unwrap()
  }

  pub fn add_counter(&mut self, other: &Counter) {
    for (&id, &count) in &other.counts {
      *self.counts.entry(id).or_insert(0) += count;
    }
  }
}
