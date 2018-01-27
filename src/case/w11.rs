use blockbuffer::world1::*;
use emu::{Emu, EmuResult, SmbEmu};
use emu::inputs::*;
use heuristics::{BoundsHeuristic, SearchGoal};
use heuristics::xpos::XPosHeuristic;
use options::*;
use state::{CompressedState, Dir, Dist, PlayerState, State};

fn w11_start<O: Options>() -> State {
  State {
    x_pos: 0x2800,
    y_pos: 0x1b000,
    x_spd: 0,
    v_force: O::Platform::V_FORCE_AREA_INIT,
    y_spd: 0,
    v_force_down: O::Platform::V_FORCE_AREA_INIT,
    facing_dir: Dir::RIGHT,
    moving_dir: Dir::empty(),
    player_state: PlayerState::STANDING,
    x_spd_abs: 0,
    running_speed: false,
    collision_bits: Dir::LR,
    side_collision_timer: 0,
    left_screen_edge_pos: 0,
    jump_swim_timer: 0,
    running_timer: 0,
    is_crouching: false,
    coin_collected: false,
    powerup_block_hit: false,
    powerup_collected: false,
  }
}

fn w11_sub_start<O: Options>() -> State {
  SmbEmu::<O, BB11Sub>::iterate_entrance(State {
    x_pos: 0x1800,
    y_pos: 0x12000,
    x_spd: 0,
    v_force: O::Platform::V_FORCE_AREA_INIT,
    y_spd: 0,
    v_force_down: O::Platform::V_FORCE_AREA_INIT,
    facing_dir: Dir::RIGHT,
    moving_dir: Dir::empty(),
    player_state: PlayerState::FALLING,
    x_spd_abs: 0,
    running_speed: false,
    collision_bits: Dir::LR,
    side_collision_timer: 0,
    left_screen_edge_pos: 0,
    jump_swim_timer: 0,
    running_timer: 0,
    is_crouching: false,
    coin_collected: false,
    powerup_block_hit: false,
    powerup_collected: false,
  })
}

#[allow(dead_code)]
pub struct W11Speedup {
  h: XPosHeuristic,
}
impl SearchGoal for W11Speedup {
  fn new() -> Self { return Self { h: XPosHeuristic::new::<<Self as super::SmbSearchCase>::Options>(&<Self as super::SmbSearchCase>::start_states()) }; }
  fn distance_to_goal_heuristic(&self, s: &State, _: Dist) -> Option<Dist> {
    Some(self.h.get_steps_until_x_pos_at_least(s, 0x9190))
  }
  fn is_goal_state(&self, s: &State, _: &EmuResult) -> bool {
    s.x_pos >= 0x9190
  }
}
impl super::SmbSearchCase for W11Speedup {
  type SearchGoal = Self;
  type StateStore = ::store::VecHashMap<CompressedState<Self::Options>, Dist>;

  type BlockBuffer = BB11;

  type CoinHandler = IgnoreCoins;
  type Platform = NTSC;
  type PlayerSize = Small;
  type PlayerSwimming = NotSwimming;
  type PowerupHandler = NoPowerups;
  type RunningTimer = NoRunningTimer;
  type ScrollPos = NoScrollPos;
  type VerticalPipeHandler = IgnoreVerticalPipes;
  type YPosFractionalBehavior = KeepYPosFractionals;

  fn start_states() -> Vec<State> {
    vec![w11_start::<Self::Options>()]
  }
  const SEARCH_SPACE_SIZE_HINT: usize = 24756;
}

#[allow(dead_code)]
pub struct W11VertPipeEntry {
  h: XPosHeuristic,
}
impl SearchGoal for W11VertPipeEntry {
  fn new() -> Self { return Self { h: XPosHeuristic::new::<<Self as super::SmbSearchCase>::Options>(&<Self as super::SmbSearchCase>::start_states()) }; }
  fn distance_to_goal_heuristic(&self, s: &State, _: Dist) -> Option<Dist> {
    Some(self.h.get_steps_until_x_pos_at_least(s, 0x39400))
  }
  fn is_goal_state(&self, _: &State, emu_result: &EmuResult) -> bool {
    if let &EmuResult::StateChangeVerticalPipe(cx, cy) = emu_result {
      cx == 0x39 && cy == 7
    } else { false }
  }
}
impl super::SmbSearchCase for W11VertPipeEntry {
  type SearchGoal = Self;
  type StateStore = ::store::VecHashMap<CompressedState<Self::Options>, Dist>;

  type BlockBuffer = BB11;

  type CoinHandler = IgnoreCoins;
  type Platform = NTSC;
  type PlayerSize = Small;
  type PlayerSwimming = NotSwimming;
  type PowerupHandler = NoPowerups;
  type RunningTimer = NoRunningTimer;
  type ScrollPos = NoScrollPos;
  type VerticalPipeHandler = EnterVerticalPipe<::typenum::U57, ::typenum::U7>;
  type YPosFractionalBehavior = KeepYPosFractionals;

  fn start_states() -> Vec<State> {
    vec![w11_start::<Self::Options>()]
  }
  const SEARCH_SPACE_SIZE_HINT: usize = 283205;
}

#[allow(dead_code)]
pub struct W11SubSpeedup {
  h: XPosHeuristic,
}
impl SearchGoal for W11SubSpeedup {
  fn new() -> Self { return Self { h: XPosHeuristic::new::<<Self as super::SmbSearchCase>::Options>(&<Self as super::SmbSearchCase>::start_states()) }; }
  fn distance_to_goal_heuristic(&self, s: &State, _: Dist) -> Option<Dist> {
    Some(self.h.get_steps_until_x_pos_at_least(s, 0x91e0))
  }
  fn is_goal_state(&self, s: &State, _: &EmuResult) -> bool {
    s.x_pos >= 0x91e0
  }
}
impl super::SmbSearchCase for W11SubSpeedup {
  type SearchGoal = Self;
  type StateStore = ::store::VecHashMap<CompressedState<Self::Options>, Dist>;

  type BlockBuffer = BB11Sub;

  type CoinHandler = IgnoreCoins;
  type Platform = NTSC;
  type PlayerSize = Small;
  type PlayerSwimming = NotSwimming;
  type PowerupHandler = NoPowerups;
  type RunningTimer = NoRunningTimer;
  type ScrollPos = NoScrollPos;
  type VerticalPipeHandler = IgnoreVerticalPipes;
  type YPosFractionalBehavior = KeepYPosFractionals;

  fn start_states() -> Vec<State> {
    vec![w11_sub_start::<Self::Options>()]
  }
  const SEARCH_SPACE_SIZE_HINT: usize = 1523576;
}

#[allow(dead_code)]
pub struct W11SubSidePipeEntry {
  heuristic: BoundsHeuristic,
}
impl SearchGoal for W11SubSidePipeEntry {
  fn new() -> Self { Self { heuristic: BoundsHeuristic::new::<<Self as super::SmbSearchCase>::Options>(&<Self as super::SmbSearchCase>::start_states()) } }
  fn distance_to_goal_heuristic(&self, s: &State, _: Dist) -> Option<Dist> {
    Some(self.heuristic.get_steps_until_bounds_at_least(s, 0xc300, 0x1b000))
  }
  fn is_goal_state(&self, _: &State, emu_result: &EmuResult) -> bool {
    if let &EmuResult::StateChangeSidePipe(cx, cy) = emu_result {
      cx == 13 && cy == 10
    } else { false }
  }
}
impl super::SmbSearchCase for W11SubSidePipeEntry {
  type SearchGoal = Self;
  type StateStore = ::store::VecHashMap<CompressedState<Self::Options>, Dist>;
  // type StateStore = ::store::VecHashMap<::state::StateDist<CompressedState<Self::Options>>, ()>;
  // type StateStore = ::std::collections::HashMap<CompressedState<Self::Options>, Dist>;

  type BlockBuffer = BB11Sub;

  type CoinHandler = IgnoreCoins;
  type Platform = NTSC;
  type PlayerSize = Small;
  type PlayerSwimming = NotSwimming;
  type PowerupHandler = NoPowerups;
  type RunningTimer = NoRunningTimer;
  type ScrollPos = NoScrollPos;
  type VerticalPipeHandler = IgnoreVerticalPipes;
  type YPosFractionalBehavior = ClearYPosFractionals;

  fn start_states() -> Vec<State> {
    vec![w11_sub_start::<Self::Options>()]
  }
  //total number of YPosStates: 17575; total size of state map: 26270
  const SEARCH_SPACE_SIZE_HINT: usize = 353165314;
  //const SEARCH_SPACE_SIZE_HINT: usize = 368488637;
}

#[allow(dead_code)]
pub enum EmulatorTesting {}
impl super::SmbCase for EmulatorTesting {
  type BlockBuffer = BB11;

  type CoinHandler = IgnoreCoins;
  type Platform = NTSC;
  type PlayerSize = Small;
  type PlayerSwimming = NotSwimming;
  type PowerupHandler = NoPowerups;
  type RunningTimer = NoRunningTimer;
  type ScrollPos = NoScrollPos;
  type VerticalPipeHandler = IgnoreVerticalPipes;
  type YPosFractionalBehavior = KeepYPosFractionals;

  fn run() -> () {
    let s = w11_start::<Self::Options>();
    let _x_heuristic = ::heuristics::xpos::XPosHeuristic::new::<Self::Options>(&vec![s.clone()]);
    let _y_heuristic = ::heuristics::ypos::YPosHeuristic::new::<Self::Options>(&vec![s.clone()]);

    let s = w11_start::<Self::Options>();
    println!("Old State: {}", s);
    let s = Self::Emu::run_steps_nr(s, &[L|R, A|R]);
    let s = Self::Emu::run_steps_nr(s, &[R; 19]);
    let s = Self::Emu::run_steps_nr(s, &[NIL, B|R, L, A|R]);
    let s = Self::Emu::run_steps_nr(s, &[R; 7]);
    let s = Self::Emu::run_steps_nr(s, &[NIL; 13]);
    println!("New State: {}", s);
  }
}