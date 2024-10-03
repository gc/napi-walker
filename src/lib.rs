#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;

pub mod builder;
pub mod table;
use std::thread;

use builder::{NewBuilder, WalkerTableBuilder};
use counter::Counter;
use napi::bindgen_prelude::{Float32Array, Int32Array};
pub mod counter;
mod util;

#[napi]
pub fn roll_walker_table(quantity: i32, index_weights: Float32Array) -> Int32Array {
  let builder = WalkerTableBuilder::new(&index_weights);
  let wa_table = builder.build();

  let num_cpus = num_cpus::get();
  let rolls_per_thread = quantity / num_cpus as i32;
  let remainder = quantity % num_cpus as i32;

  let mut handles: Vec<thread::JoinHandle<Counter>> = vec![];

  for _ in 0..num_cpus {
    let clone = wa_table.clone();
    handles.push(thread::spawn(move || clone.next(rolls_per_thread)));
  }

  let mut result = Counter::new();
  for handle in handles {
    let loot = handle.join().unwrap();
    result.add_counter(&loot);
  }

  if remainder > 0 {
    result.add_counter(&wa_table.next(remainder));
  }

  result.to_js()
}
