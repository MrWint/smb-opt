use blockbuffer::BlockBuffer;
use emu::{Emu,SmbEmu,print_rle};
use heuristics::SearchGoal;
use ida::{IDA, InputFetcher, Search, SearchResult, SmbInputFetcher};
use options::*;
use state::{Dir, Dist, State};
use store::StateStore;

pub mod w11;
pub mod w12;
pub mod w13;
pub mod w14;
pub mod w21;

#[allow(dead_code)]
pub fn with_left_and_right_facing_dir(states: Vec<State>) -> Vec<State> {
  states.into_iter().flat_map(|s| {
    (&[Dir::LEFT, Dir::RIGHT]).iter().map(move |d| {
      let mut s = s.clone();
      s.facing_dir = *d;
      s
    })
  }).collect()
}

#[allow(dead_code)]
pub fn with_left_and_right_and_lr_facing_dir(states: Vec<State>) -> Vec<State> {
  states.into_iter().flat_map(|s| {
    (&[Dir::LEFT, Dir::RIGHT, Dir::LR]).iter().map(move |d| {
      let mut s = s.clone();
      s.facing_dir = *d;
      s
    })
  }).collect()
}

#[allow(dead_code)]
pub fn with_smaller_x_pos<O: Options>(states: Vec<State>, num_subpixels: i32) -> Vec<State> {
  states.into_iter().flat_map(|s| {
    (0..num_subpixels+1).map(move |d| {
      let mut s = s.clone();
      s.x_pos -= d << 4;
      if O::ScrollPos::TRACK_SCROLL_POS { s.left_screen_edge_pos = ((s.left_screen_edge_pos as i32 - (s.x_pos >> 8) + (s.x_pos >> 8)) & 0xff) as u8; }
      s
    })
  }).collect()
}

#[allow(dead_code)]
pub fn with_all_x_spd_subpixels(states: Vec<State>) -> Vec<State> {
  states.into_iter().flat_map(|s| {
    (0..0x40).map(move |d| {
      let mut s = s.clone();
      s.x_spd = (s.x_spd & !0xff) + (d << 2);
      s
    })
  }).collect()
}

#[allow(dead_code)]
pub fn with_all_y_pos_subpixels(states: Vec<State>) -> Vec<State> {
  states.into_iter().flat_map(|s| {
    (0..0x80).map(move |d| {
      let mut s = s.clone();
      s.y_pos = (s.y_pos & !0xff) + (d << 1);
      s
    })
  }).collect()
}

pub trait Case {
  fn run() -> ();
}
pub trait SmbCase: Options + Sized {
  type BlockBuffer: BlockBuffer;

  type Emu: Emu = SmbEmu<Self, Self::BlockBuffer>;

  fn run() -> ();
}
impl<T: SmbCase> Case for T {
  fn run() -> () {
    Self::run();
  }
}

pub trait SmbSearchCase: Options + Sized {
  type SearchGoal: SearchGoal;
  type StateStore: StateStore;

  type BlockBuffer: BlockBuffer;

  type Emu: Emu = SmbEmu<Self, Self::BlockBuffer>;
  type InputFetcher: InputFetcher = SmbInputFetcher<Self>;
  type Search: Search = IDA<Self::StateStore, Self::Emu, Self::SearchGoal, Self::InputFetcher>;

  fn start_states() -> Vec<State>;
  const INITIAL_SEARCH_DISTANCE: Dist = 0;
  const SEARCH_SPACE_SIZE_HINT: usize = 10;
}

impl<T: SmbSearchCase> SmbCase for T {
  type BlockBuffer = T::BlockBuffer;

  fn run() -> () {
    if let SearchResult::Found(states, inputs) = T::Search::find_first_solution(Self::start_states(), Self::INITIAL_SEARCH_DISTANCE, Self::SEARCH_SPACE_SIZE_HINT) {
      println!("Found solution!");
      // println!("State sequence:"); for i in 0..states.len() { println!("{}: {}", i, states[i]); }
      println!("Initial state: {}", states[0]);
      println!("Final state: {}", states[states.len() - 1]);
      print!("Input sequence: ");
      print_rle(inputs.iter());
    } else {
      println!("No solutions found!");
    }
  }
}