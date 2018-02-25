#![feature(associated_type_defaults)]
#![feature(fixed_size_array)]

#[macro_use]
extern crate bitflags;
extern crate bitpack;
extern crate core;
#[macro_use]
extern crate lazy_static;
extern crate time;
extern crate typenum;

use case::Case;

mod blockbuffer;
mod case;
mod emu;
mod heuristics;
mod ida;
mod options;
mod state;
mod store;

fn main() {
  // calc_prng();
  case::w31::W31ScreenScrollBlock3PipeClip::run();
  // case::w21::W21ScreenScrollBlock3PipeClip::run();
}

#[allow(dead_code)]
fn calc_prng() {
  let mut result: Vec<u8> = vec![0xa0];
  let mut state_map: ::std::collections::HashMap<u16, usize> = ::std::collections::HashMap::new();
  let mut cur_step: usize = 0;
  let mut cur_state: u16 = 0xa500;
  while !state_map.contains_key(&cur_state) {
    state_map.insert(cur_state, cur_step);

    cur_state = (((cur_state << 6) ^ (cur_state << 14)) & 0x8000) | ((cur_state >> 1) & 0x7ffe);
    cur_step += 1;
    if cur_step % 8 == 3 {
      result.insert(0, (cur_state >> 8) as u8);
    }
  }
  println!("found cycle after {} steps!", cur_step);
  print!("Cycle: ");
  for b in result { print!("{:02x}", b) }
  println!();
}