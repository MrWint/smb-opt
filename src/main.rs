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
  case::w21::W21ScreenScrollBlock3PipeClip::run();
}
