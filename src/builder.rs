use crate::table::WalkerTable;
use crate::util::math::gcd_for_slice;

pub trait NewBuilder<T> {
  fn new(index_weights: &[T]) -> WalkerTableBuilder;
}

pub struct WalkerTableBuilder {
  index_weights: Vec<u32>,
}

impl NewBuilder<u32> for WalkerTableBuilder {
  fn new(index_weights: &[u32]) -> WalkerTableBuilder {
    let table_len = index_weights.len() as u32;

    // Process that the mean of index_weights does not become a float value
    let ws = index_weights
      .iter()
      .map(|w| w * table_len)
      .collect::<Vec<u32>>();

    WalkerTableBuilder { index_weights: ws }
  }
}

impl NewBuilder<f32> for WalkerTableBuilder {
  fn new(index_weights: &[f32]) -> WalkerTableBuilder {
    let ws = index_weights
      .iter()
      .map(|w| (w * 10000.0).round() as u32)
      .collect::<Vec<u32>>();

    let gcd = gcd_for_slice(&ws);
    let ws = ws.iter().map(|w| w / gcd).collect::<Vec<u32>>();

    WalkerTableBuilder::new(&ws)
  }
}

impl WalkerTableBuilder {
  /// Builds a new instance of [`WalkerTable`].
  pub fn build(&self) -> WalkerTable {
    let table_len = self.index_weights.len();

    if self.sum() == 0 {
      // Returns WalkerTable that performs unweighted random sampling.
      return WalkerTable::new(vec![0; table_len], vec![0.0; table_len]);
    }

    let (aliases, probs) = self.calc_table();

    WalkerTable::new(aliases, probs)
  }

  /// Calculates the sum of `index_weights`.
  fn sum(&self) -> u32 {
    self.index_weights.iter().fold(0, |acc, cur| acc + cur)
  }

  /// Calculates the mean of `index_weights`.
  fn mean(&self) -> u32 {
    self.sum() / self.index_weights.len() as u32
  }

  /// Returns the tables of aliases and probabilities.
  fn calc_table(&self) -> (Vec<usize>, Vec<f32>) {
    let table_len = self.index_weights.len();
    let (mut below_vec, mut above_vec) = self.separate_weight();
    let mean = self.mean();

    let mut aliases = vec![0; table_len];
    let mut probs = vec![0.0; table_len];
    loop {
      match below_vec.pop() {
        Some(below) => {
          if let Some(above) = above_vec.pop() {
            let diff = mean - below.1;
            aliases[below.0] = above.0 as usize;
            probs[below.0] = diff as f32 / mean as f32;
            if above.1 - diff <= mean {
              below_vec.push((above.0, above.1 - diff));
            } else {
              above_vec.push((above.0, above.1 - diff));
            }
          } else {
            aliases[below.0] = below.0;
            probs[below.0] = below.1 as f32 / mean as f32;
          }
        }
        None => break,
      }
    }

    (aliases, probs)
  }

  /// Divide the values of `index_weights` based on the mean of them.
  ///
  /// The tail value is a weight and head is its index.
  fn separate_weight(&self) -> (Vec<(usize, u32)>, Vec<(usize, u32)>) {
    let mut below_vec = Vec::with_capacity(self.index_weights.len());
    let mut above_vec = Vec::with_capacity(self.index_weights.len());
    for (i, w) in self.index_weights.iter().enumerate() {
      if *w <= self.mean() {
        below_vec.push((i, *w));
      } else {
        above_vec.push((i, *w));
      }
    }
    (below_vec, above_vec)
  }
}
