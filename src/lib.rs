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

  let mut result = Counter::new();
  for handle in handles {
    let loot = handle.join().unwrap();
    result.add_counter(&loot);
  }

  result.to_json()
}
