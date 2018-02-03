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

fn w11_pipe_start<O: Options>() -> State {
  State {
    x_pos: 0xa3800,
    y_pos: 0x19000,
    x_spd: 0x0,
    v_force: O::Platform::V_FORCE_AREA_INIT,
    y_spd: 0x0,
    v_force_down: O::Platform::V_FORCE_AREA_INIT,
    facing_dir: Dir::RIGHT,
    moving_dir: Dir::empty(),
    player_state: PlayerState::STANDING,
    x_spd_abs: 0,
    running_speed: false,
    collision_bits: Dir::LR,
    side_collision_timer: 0,
    left_screen_edge_pos: 0x0,
    jump_swim_timer: 0,
    running_timer : 0,
    is_crouching: false,
    coin_collected: false,
    powerup_block_hit: false,
    powerup_collected: false,
  }
}

/// Initial speed-up starting in 1-1
/// Input sequence: [1x L|R, 1x A|R, 19x R, 1x NIL, 1x B|R, 1x L, 1x A|R, 20x R, 14x B|R] (len: 59)
#[allow(dead_code)]
pub struct W11Speedup {
  h: XPosHeuristic,
}
impl SearchGoal for W11Speedup {
  fn new() -> Self { return Self { h: XPosHeuristic::new::<<Self as super::SmbSearchCase>::Options>(&<Self as super::SmbSearchCase>::start_states()) }; }
  fn distance_to_goal_heuristic(&self, s: &mut State, _: Dist) -> Option<Dist> {
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
  const SEARCH_SPACE_SIZE_HINT: usize = 59;
}

/// Start of 1-1 until enter pipe to sub world
/// Input sequence: [1x L|R, 1x A, 20x R, 2x B|R, 1x L, 1x A|R, 20x R, 124x B|R, 2x A, 9x R, 12x B|R, 13x R, 24x B|R, 6x A, 8x R, 13x B|R, 16x R, 3x B|R, 12x A, 6x R, 14x B|R, 21x R, 18x B|R, 12x A, 6x R, 2x B|R, 1x NIL] (len: 368)
#[allow(dead_code)]
pub struct W11VertPipeEntry {
  h: XPosHeuristic,
}
impl SearchGoal for W11VertPipeEntry {
  fn new() -> Self { return Self { h: XPosHeuristic::new::<<Self as super::SmbSearchCase>::Options>(&<Self as super::SmbSearchCase>::start_states()) }; }
  fn distance_to_goal_heuristic(&self, s: &mut State, _: Dist) -> Option<Dist> {
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

/// Initial speed-up starting in 1-1-sub
/// Input sequence: [1x NIL, 2x L, 2x NIL, 29x R, 10x A|R, 4x R, 4x NIL, 1x B|R, 1x L, 1x A|R, 20x R, 13x B|R] (len: 88)
#[allow(dead_code)]
pub struct W11SubSpeedup {
  h: XPosHeuristic,
}
impl SearchGoal for W11SubSpeedup {
  fn new() -> Self { return Self { h: XPosHeuristic::new::<<Self as super::SmbSearchCase>::Options>(&<Self as super::SmbSearchCase>::start_states()) }; }
  fn distance_to_goal_heuristic(&self, s: &mut State, _: Dist) -> Option<Dist> {
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

/// Start of 1-1-sub until entering side pipe
/// Input sequence: [6x R, 1x NIL, 2x L, 3x NIL, 22x R, 1x A|L, 9x A|R, 7x R, 1x NIL, 1x B|R, 1x L, 1x A|R, 20x R, 3x B|R, 1x A, 25x R, 3x L, 1x NIL, 3x R] (len: 111)
#[allow(dead_code)]
pub struct W11SubSidePipeEntry {
  heuristic: BoundsHeuristic,
}
impl SearchGoal for W11SubSidePipeEntry {
  fn new() -> Self { Self { heuristic: BoundsHeuristic::new::<<Self as super::SmbSearchCase>::Options>(&<Self as super::SmbSearchCase>::start_states()) } }
  fn distance_to_goal_heuristic(&self, s: &mut State, _: Dist) -> Option<Dist> {
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
  const SEARCH_SPACE_SIZE_HINT: usize = 368488637;
}

/// Initial speed-up starting in 1-1-pipe
/// Input sequence: [1x L|R, 1x A|R, 19x R, 1x NIL, 1x B|R, 1x L, 13x R, 68x B|R, 2x A, 9x R, 12x B|R, 3x R, 4x B|R, 1x A, 9x R, 3x B|R, 1x A, 9x R, 2x B|R, 1x A, 9x R, 4x B|R] (len: 174)
#[allow(dead_code)]
pub struct W11PipeSpeedup {
  h: XPosHeuristic,
}
impl SearchGoal for W11PipeSpeedup {
  fn new() -> Self { return Self { h: XPosHeuristic::new::<<Self as super::SmbSearchCase>::Options>(&<Self as super::SmbSearchCase>::start_states()) }; }
  fn distance_to_goal_heuristic(&self, s: &mut State, _: Dist) -> Option<Dist> {
    Some(self.h.get_steps_until_x_pos_at_least(s, 0xbc110))
  }
  fn is_goal_state(&self, s: &State, _: &EmuResult) -> bool {
    s.x_pos >= 0xbc110
  }
}
impl super::SmbSearchCase for W11PipeSpeedup {
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
    vec![w11_pipe_start::<Self::Options>()]
  }
  const SEARCH_SPACE_SIZE_HINT: usize = 266;
}

/// Stairs to half flag pole glitch
/// Input sequence: [3x B|R, 1x A, 42x R, 2x L, 3x NIL, 2x R, 1x L, 1x NIL, 1x R, 1x A|L, 1x NIL, 1x R] (len: 59)
#[allow(dead_code)]
pub struct W11Flag {
  heuristic: BoundsHeuristic,
}
impl SearchGoal for W11Flag {
  fn new() -> Self { Self { heuristic: BoundsHeuristic::new::<<Self as super::SmbSearchCase>::Options>(&<Self as super::SmbSearchCase>::start_states()) } }
  fn distance_to_goal_heuristic(&self, s: &mut State, _: Dist) -> Option<Dist> {
    Some(self.heuristic.get_steps_until_bounds_at_least(s, 0xc5600, 0x1a200))
  }
  fn is_goal_state(&self, s: &State, emu_result: &EmuResult) -> bool {
    if let &EmuResult::StateChangeFlag(cx, cy) = emu_result {
      cx == 0xc6 && cy == 9 && s.y_pos >= 0x1a200 && (s.x_pos & 0xf0) >= 0x70
    } else { false }
  }
}
impl super::SmbSearchCase for W11Flag {
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
    let s = State {
      x_pos: 0xbb490,
      y_pos: 0x130e0,
      x_spd: 0x28f4,
      v_force: Self::Platform::V_FORCE_FALL_RUNNING,
      y_spd: 0x0,
      v_force_down: Self::Platform::V_FORCE_FALL_RUNNING,
      facing_dir: Dir::RIGHT,
      moving_dir: Dir::RIGHT,
      player_state: PlayerState::STANDING,
      x_spd_abs: 33,
      running_speed: true,
      collision_bits: Dir::LR,
      side_collision_timer: 0,
      left_screen_edge_pos: 0x44,
      jump_swim_timer: 21,
      running_timer : 0,
      is_crouching: false,
      coin_collected: false,
      powerup_block_hit: false,
      powerup_collected: false,
    };
    let s = Self::Emu::run_steps_nr(s, &[B|R; 12]);
    vec![s]
  }
  const INITIAL_SEARCH_DISTANCE: Dist = 59;
  const SEARCH_SPACE_SIZE_HINT: usize = 83848968; // initial dist 0: 165497479;
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