#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;

pub mod builder;
pub mod table;
use std::{sync::Arc, thread};

use builder::{NewBuilder, WalkerTableBuilder};
use counter::Counter;
use napi::bindgen_prelude::Float32Array;
pub mod counter;
mod util;

#[napi]
pub fn roll_walker_table(quantity: i32, index_weights: Float32Array) -> String {
  let builder = WalkerTableBuilder::new(&index_weights);
  let wa_table = Arc::new(builder.build());

  let mut result = Counter::new();

  if index_weights.len() == 1 {
    result.add(0, quantity);
    return result.to_json();
  }

  let num_cpus = num_cpus::get();
  let rolls_per_thread = quantity / num_cpus as i32;
  let remainder = quantity % num_cpus as i32;

  let mut handles: Vec<thread::JoinHandle<Counter>> = vec![];

  for index in 0..num_cpus {
    let qty = if index == 0 {
      rolls_per_thread + remainder
    } else {
      rolls_per_thread
    };
    let table = wa_table.clone();
    handles.push(thread::spawn(move || table.next(qty)));
  }

  for handle in handles {
    let loot = handle.join().unwrap();
    result.add_counter(&loot);
  }

  result.to_json()
}

#[napi]
pub fn simulate_chances(rolls: u32, chance: u16) -> u32 {
  let mut success_count = 0;
  let mut rng = fastrand::Rng::new();
  for _ in 0..rolls {
    if rng.u16(0..chance) == 0 {
      success_count += 1;
    }
  }
  success_count
}
