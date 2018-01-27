#![feature(associated_type_defaults)]

#[macro_use]
extern crate bitflags;
extern crate bitpack;
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
  case::w11::W11SubSidePipeEntry::run();
  // case::w11::EmulatorTesting::run();
}
