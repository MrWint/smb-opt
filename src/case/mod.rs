use blockbuffer::BlockBuffer;
use emu::{Emu,SmbEmu,print_rle};
use heuristics::SearchGoal;
use ida::{IDA, InputFetcher, Search, SearchResult, SmbInputFetcher};
use options::*;
use state::State;
use store::StateStore;

pub mod w11;

pub trait Case {
  fn run() -> ();
}
pub trait SmbCase {
  type BlockBuffer: BlockBuffer;

  type CoinHandler: CoinHandler;
  type Platform: Platform;
  type PlayerSize: PlayerSize;
  type PlayerSwimming: PlayerSwimming;
  type PowerupHandler: PowerupHandler;
  type RunningTimer: RunningTimer;
  type ScrollPos: ScrollPos;
  type VerticalPipeHandler: VerticalPipeHandler;
  type YPosFractionalBehavior: YPosFractionalBehavior;

  type Options: Options = SmbOptions<Self::PlayerSize, Self::PlayerSwimming, Self::RunningTimer, Self::YPosFractionalBehavior, Self::ScrollPos, Self::Platform, Self::CoinHandler, Self::PowerupHandler, Self::VerticalPipeHandler>;
  type Emu: Emu = SmbEmu<Self::Options, Self::BlockBuffer>;

  fn run() -> ();
}
impl<T: SmbCase> Case for T {
  fn run() -> () {
    Self::run();
  }
}

pub trait SmbSearchCase {
  type SearchGoal: SearchGoal;
  type StateStore: StateStore;

  type BlockBuffer: BlockBuffer;

  type CoinHandler: CoinHandler;
  type Platform: Platform;
  type PlayerSize: PlayerSize;
  type PlayerSwimming: PlayerSwimming;
  type PowerupHandler: PowerupHandler;
  type RunningTimer: RunningTimer;
  type ScrollPos: ScrollPos;
  type VerticalPipeHandler: VerticalPipeHandler;
  type YPosFractionalBehavior: YPosFractionalBehavior;

  type Options: Options = SmbOptions<Self::PlayerSize, Self::PlayerSwimming, Self::RunningTimer, Self::YPosFractionalBehavior, Self::ScrollPos, Self::Platform, Self::CoinHandler, Self::PowerupHandler, Self::VerticalPipeHandler>;
  type Emu: Emu = SmbEmu<Self::Options, Self::BlockBuffer>;
  type InputFetcher: InputFetcher = SmbInputFetcher<Self::Options>;
  type Search: Search = IDA<Self::StateStore, Self::Emu, Self::SearchGoal, Self::InputFetcher>;

  fn start_states() -> Vec<State>;
  const SEARCH_SPACE_SIZE_HINT: usize;
}

impl<T: SmbSearchCase> SmbCase for T {
  type BlockBuffer = T::BlockBuffer;

  type CoinHandler = T::CoinHandler;
  type Platform = T::Platform;
  type PlayerSize = T::PlayerSize;
  type PlayerSwimming = T::PlayerSwimming;
  type PowerupHandler = T::PowerupHandler;
  type RunningTimer = T::RunningTimer;
  type ScrollPos = T::ScrollPos;
  type VerticalPipeHandler = T::VerticalPipeHandler;
  type YPosFractionalBehavior = T::YPosFractionalBehavior;

  fn run() -> () {
    if let SearchResult::Found(_states, inputs) = T::Search::find_first_solution(Self::start_states(), Self::SEARCH_SPACE_SIZE_HINT) {
      println!("Found solution!");
      //println!("State sequence: {:?}", states);
      print!("Input sequence: ");
      print_rle(inputs.iter());
    } else {
      println!("No solutions found!");
    }
  }
}