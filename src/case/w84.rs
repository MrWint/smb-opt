use blockbuffer::world8::*;
use emu::EmuResult;
#[allow(unused_imports)] use emu::inputs::*;
use heuristics::SearchGoal;
use heuristics::xpos::XPosHeuristic;
use options::*;
use state::{CompressedState, Dir, Dist, PlayerState, State};


/// Initial speed-up starting in 8-4
/// Input sequence: [1x L|R, 1x A|R, 19x R, 9x NIL, 1x B|R, 1x L, 2x A|R, 37x R, 8x B|R] (len: 79)
#[allow(dead_code)]
pub struct W84Speedup {
  max_x_pos: i32,
  h: XPosHeuristic,
}
impl Options for W84Speedup {
  type CoinHandler = IgnoreCoins;
  type Platform = NTSC;
  type PlayerSize = Big;
  type Swim = NotSwimming;
  type PowerupHandler = NoPowerups;
  type RunningTimer = NoRunningTimer;
  type ScrollPos = NoScrollPos;
  type Parity = NoParity;
  type VerticalPipeHandler = IgnoreVerticalPipes;
  type YPosFractionalBehavior = KeepYPosFractionals;
}
impl super::SmbSearchCase for W84Speedup {
  type SearchGoal = Self;
  type StateStore = ::store::VecHashMap<CompressedState<Self, [u8; 10]>, Dist>;

  type BlockBuffer = BB84;

  fn start_states() -> Vec<State> {
    let s = State {
      x_pos: 0x2800,
      y_pos: 0x15000,
      x_spd: 0x0,
      y_spd: 0x0,
      player_state: PlayerState::STANDING,
      moving_dir: Dir::empty(),
      facing_dir: Dir::RIGHT,
      v_force: <Self as Options>::Platform::V_FORCE_AREA_INIT,
      v_force_down: <Self as Options>::Platform::V_FORCE_AREA_INIT,
      x_spd_abs: 0x0,
      running_speed: false,
      collision_bits: Dir::LR,
      is_crouching: false,
      jump_swim_timer: 0,
      running_timer : 0,
      left_screen_edge_pos: 0x0,
      side_collision_timer: 0,
      collected_coins: 0,
      powerup_block_hit: false,
      powerup_collected: false,
      parity: 0,
    };
    vec![s]
  }
  const INITIAL_SEARCH_DISTANCE: Dist = 79;
  const SEARCH_SPACE_SIZE_HINT: usize = 800000000;
}
impl SearchGoal for W84Speedup {
  fn new() -> Self { return Self { max_x_pos: 0, h: XPosHeuristic::new::<Self>(&<Self as super::SmbSearchCase>::start_states()) }; }
  fn distance_to_goal_heuristic(&self, s: &mut State, _: Dist) -> Option<Dist> {
    // Some(self.h.get_steps_until_x_pos_at_least(s, 0x9270))
    Some(self.h.get_steps_until_x_pos_at_least(s, self.max_x_pos + 0x10))
  }
  fn is_goal_state(&self, _s: &State, _: &EmuResult) -> bool {
    _s.x_pos >= 0xbb90
    // false
  }
  fn track_metric(&mut self, s: &State) -> () {
    if self.max_x_pos < s.x_pos {
      self.max_x_pos = s.x_pos;
      println!("new best max_x_pos: {:x}", self.max_x_pos);
    }
  }
  fn report_metrics(&self) -> () {
    println!("best max_x_pos so far: {:x}", self.max_x_pos);
  }
}


/// Part 2 speed-up starting in 8-4
/// Input sequence: [1x L|R, 1x A|R, 19x R, 1x NIL, 1x B|R, 1x L, 13x R, 7x B|R, 6x A, 8x R, 12x B|R] (len: 70)
#[allow(dead_code)]
pub struct W84Part2Speedup {
  max_x_pos: i32,
  h: XPosHeuristic,
}
impl Options for W84Part2Speedup {
  type CoinHandler = IgnoreCoins;
  type Platform = NTSC;
  type PlayerSize = Big;
  type Swim = NotSwimming;
  type PowerupHandler = NoPowerups;
  type RunningTimer = NoRunningTimer;
  type ScrollPos = NoScrollPos;
  type Parity = NoParity;
  type VerticalPipeHandler = IgnoreVerticalPipes;
  type YPosFractionalBehavior = KeepYPosFractionals;
}
impl super::SmbSearchCase for W84Part2Speedup {
  type SearchGoal = Self;
  type StateStore = ::store::VecHashMap<CompressedState<Self, [u8; 10]>, Dist>;

  type BlockBuffer = BB84;

  fn start_states() -> Vec<State> {
    let s = State {
      x_pos: 0x73800,
      y_pos: 0x19000,
      x_spd: 0x0,
      y_spd: 0x0,
      player_state: PlayerState::STANDING,
      moving_dir: Dir::empty(),
      facing_dir: Dir::RIGHT,
      v_force: <Self as Options>::Platform::V_FORCE_AREA_INIT,
      v_force_down: <Self as Options>::Platform::V_FORCE_AREA_INIT,
      x_spd_abs: 0x0,
      running_speed: false,
      collision_bits: Dir::LR,
      is_crouching: false,
      jump_swim_timer: 0,
      running_timer : 0,
      left_screen_edge_pos: 0x0,
      side_collision_timer: 0,
      collected_coins: 0,
      powerup_block_hit: false,
      powerup_collected: false,
      parity: 0,
    };
    vec![s]
  }
  const INITIAL_SEARCH_DISTANCE: Dist = 70;
  const SEARCH_SPACE_SIZE_HINT: usize = 800000000;
}
impl SearchGoal for W84Part2Speedup {
  fn new() -> Self { return Self { max_x_pos: 0, h: XPosHeuristic::new::<Self>(&<Self as super::SmbSearchCase>::start_states()) }; }
  fn distance_to_goal_heuristic(&self, s: &mut State, _: Dist) -> Option<Dist> {
    Some(self.h.get_steps_until_x_pos_at_least(s, 0x7bd10))
    // Some(self.h.get_steps_until_x_pos_at_least(s, self.max_x_pos + 0x10))
  }
  fn is_goal_state(&self, _s: &State, _: &EmuResult) -> bool {
    _s.x_pos >= 0x7bd10
    // false
  }
  fn track_metric(&mut self, s: &State) -> () {
    if self.max_x_pos < s.x_pos {
      self.max_x_pos = s.x_pos;
      println!("new best max_x_pos: {:x}", self.max_x_pos);
    }
  }
  fn report_metrics(&self) -> () {
    println!("best max_x_pos so far: {:x}", self.max_x_pos);
  }
}


/// Part 2 8-4 pipe clip entry
/// Input sequence: [1x A, 7x R, 8x NIL, 1x L, 1x NIL, 1x L, 2x A, 9x R, 4x L|R, 1x B|R, 1x L|R, 3x B|R, 1x NIL] (len: 40)
#[allow(dead_code)]
pub struct W84Part2VertPipeEntry {
  max_x_pos: i32,
  h: XPosHeuristic,
}
impl Options for W84Part2VertPipeEntry {
  type CoinHandler = IgnoreCoins;
  type Platform = NTSC;
  type PlayerSize = Big;
  type Swim = NotSwimming;
  type PowerupHandler = NoPowerups;
  type RunningTimer = NoRunningTimer;
  type ScrollPos = NoScrollPos;
  type Parity = NoParity;
  type VerticalPipeHandler = EnterVerticalPipe<::typenum::U152, ::typenum::U4>;
  type YPosFractionalBehavior = KeepYPosFractionals;
}
impl super::SmbSearchCase for W84Part2VertPipeEntry {
  type SearchGoal = Self;
  type StateStore = ::store::VecHashMap<CompressedState<Self, [u8; 10]>, Dist>;

  type BlockBuffer = BB84;

  fn start_states() -> Vec<State> {
    let s = State {
      x_pos: 0x94530,
      y_pos: 0x12df8,
      x_spd: 0x2814,
      y_spd: -0x50, // 0xffb0,
      player_state: PlayerState::JUMPING,
      moving_dir: Dir::RIGHT,
      facing_dir: Dir::LEFT,
      v_force: <Self as Options>::Platform::V_FORCE_AREA_INIT,
      v_force_down: <Self as Options>::Platform::V_FORCE_FALL_RUNNING,
      x_spd_abs: 0x21,
      running_speed: true,
      collision_bits: Dir::LR,
      is_crouching: false,
      jump_swim_timer: 2,
      running_timer : 0,
      left_screen_edge_pos: 0xd5,
      side_collision_timer: 0,
      collected_coins: 0,
      powerup_block_hit: false,
      powerup_collected: false,
      parity: 0,
    };
    // super::with_smaller_x_pos::<Self>(super::with_left_and_right_facing_dir(super::with_all_x_spd_subpixels(
    vec![s]
    // )), 16)
  }
  const INITIAL_SEARCH_DISTANCE: Dist = 37;
  const SEARCH_SPACE_SIZE_HINT: usize = 800000000;
}
impl SearchGoal for W84Part2VertPipeEntry {
  fn new() -> Self { return Self { max_x_pos: 0, h: XPosHeuristic::new::<Self>(&<Self as super::SmbSearchCase>::start_states()) }; }
  fn distance_to_goal_heuristic(&self, s: &mut State, _: Dist) -> Option<Dist> {
    Some(self.h.get_steps_until_x_pos_at_least(s, 0x98400))
    // Some(self.h.get_steps_until_x_pos_at_least(s, self.max_x_pos + 0x10))
  }
  fn is_goal_state(&self, _s: &State, emu_result: &EmuResult) -> bool {
    _s.x_pos >= 0x985c0 &&
    if let &EmuResult::StateChangeVerticalPipe(cx, cy) = emu_result {
      cx == 0x98 && cy == 4
    } else { false }
  }
  fn track_metric(&mut self, s: &State) -> () {
    if self.max_x_pos < s.x_pos {
      self.max_x_pos = s.x_pos;
      println!("new best max_x_pos: {:x}", self.max_x_pos);
    }
  }
  fn report_metrics(&self) -> () {
    println!("best max_x_pos so far: {:x}", self.max_x_pos);
  }
}


/// Part 3 speed-up starting in 8-4
/// Input sequence: [1x L|R, 1x A|R, 14x R, 1x NIL, 5x R, 1x B|R, 1x L, 1x A|R, 16x R, 20x B|R, 2x A, 7x R] (len: 70)
#[allow(dead_code)]
pub struct W84Part3Speedup {
  max_x_pos: i32,
  h: XPosHeuristic,
}
impl Options for W84Part3Speedup {
  type CoinHandler = IgnoreCoins;
  type Platform = NTSC;
  type PlayerSize = Big;
  type Swim = NotSwimming;
  type PowerupHandler = NoPowerups;
  type RunningTimer = NoRunningTimer;
  type ScrollPos = NoScrollPos;
  type Parity = NoParity;
  type VerticalPipeHandler = IgnoreVerticalPipes;
  type YPosFractionalBehavior = KeepYPosFractionals;
}
impl super::SmbSearchCase for W84Part3Speedup {
  type SearchGoal = Self;
  type StateStore = ::store::VecHashMap<CompressedState<Self, [u8; 10]>, Dist>;

  type BlockBuffer = BB84;

  fn start_states() -> Vec<State> {
    let s = State {
      x_pos: 0xc3800,
      y_pos: 0x19000,
      x_spd: 0x0,
      y_spd: 0x0,
      player_state: PlayerState::STANDING,
      moving_dir: Dir::empty(),
      facing_dir: Dir::RIGHT,
      v_force: <Self as Options>::Platform::V_FORCE_AREA_INIT,
      v_force_down: <Self as Options>::Platform::V_FORCE_AREA_INIT,
      x_spd_abs: 0x0,
      running_speed: false,
      collision_bits: Dir::LR,
      is_crouching: false,
      jump_swim_timer: 0,
      running_timer : 0,
      left_screen_edge_pos: 0x0,
      side_collision_timer: 0,
      collected_coins: 0,
      powerup_block_hit: false,
      powerup_collected: false,
      parity: 0,
    };
    vec![s]
  }
  const INITIAL_SEARCH_DISTANCE: Dist = 70;
  const SEARCH_SPACE_SIZE_HINT: usize = 800000000;
}
impl SearchGoal for W84Part3Speedup {
  fn new() -> Self { return Self { max_x_pos: 0, h: XPosHeuristic::new::<Self>(&<Self as super::SmbSearchCase>::start_states()) }; }
  fn distance_to_goal_heuristic(&self, s: &mut State, _: Dist) -> Option<Dist> {
    Some(self.h.get_steps_until_x_pos_at_least(s, 0xcbcc0))
    // Some(self.h.get_steps_until_x_pos_at_least(s, self.max_x_pos + 0x10))
  }
  fn is_goal_state(&self, _s: &State, _: &EmuResult) -> bool {
    _s.x_pos >= 0xcbcc0
    // false
  }
  fn track_metric(&mut self, s: &State) -> () {
    if self.max_x_pos < s.x_pos {
      self.max_x_pos = s.x_pos;
      println!("new best max_x_pos: {:x}", self.max_x_pos);
    }
  }
  fn report_metrics(&self) -> () {
    println!("best max_x_pos so far: {:x}", self.max_x_pos);
  }
}


/// Part 3 8-4 pipe clip entry
/// Input sequence: [17x NIL, 9x L, 8x NIL, 1x L|R, 1x A|L, 1x L, 2x R, 17x L, 1x B|L, 6x A|L, 8x L, 2x B|L, 1x NIL] (len: 74)
#[allow(dead_code)]
pub struct W84Part3VertPipeEntry {
  min_x_pos: i32,
  h: XPosHeuristic,
}
impl Options for W84Part3VertPipeEntry {
  type CoinHandler = IgnoreCoins;
  type Platform = NTSC;
  type PlayerSize = Big;
  type Swim = NotSwimming;
  type PowerupHandler = NoPowerups;
  type RunningTimer = NoRunningTimer;
  type ScrollPos = WithScrollPos;
  type Parity = NoParity;
  type VerticalPipeHandler = EnterVerticalPipe<::typenum::U212, ::typenum::U5>;
  type YPosFractionalBehavior = KeepYPosFractionals;
}
impl super::SmbSearchCase for W84Part3VertPipeEntry {
  type SearchGoal = Self;
  type StateStore = ::store::VecHashMap<CompressedState<Self, [u8; 12]>, Dist>;

  type BlockBuffer = BB84;

  fn start_states() -> Vec<State> {
    let s = State {
      x_pos: 0xd3c40,
      y_pos: 0x131e0,
      x_spd: 0x2804,
      y_spd: -0x118, // 0xfee8,
      player_state: PlayerState::JUMPING,
      moving_dir: Dir::RIGHT,
      facing_dir: Dir::LEFT,
      v_force: <Self as Options>::Platform::V_FORCE_AREA_INIT,
      v_force_down: <Self as Options>::Platform::V_FORCE_FALL_RUNNING,
      x_spd_abs: 0x21,
      running_speed: true,
      collision_bits: Dir::LR,
      is_crouching: false,
      jump_swim_timer: 7,
      running_timer : 0,
      left_screen_edge_pos: 0xcc,
      side_collision_timer: 0,
      collected_coins: 0,
      powerup_block_hit: false,
      powerup_collected: false,
      parity: 0,
    };
    // super::with_smaller_x_pos::<Self>(super::with_left_and_right_facing_dir(super::with_all_x_spd_subpixels(
    vec![s]
    // )), 16)
  }
  const INITIAL_SEARCH_DISTANCE: Dist = 74;
  const SEARCH_SPACE_SIZE_HINT: usize = 800000000;
}
impl SearchGoal for W84Part3VertPipeEntry {
  fn new() -> Self { return Self { min_x_pos: 0xd4e80, h: XPosHeuristic::new::<Self>(&<Self as super::SmbSearchCase>::start_states()) }; }
  fn distance_to_goal_heuristic(&self, s: &mut State, _: Dist) -> Option<Dist> {
    if (s.left_screen_edge_pos < 0x11 || s.left_screen_edge_pos >= 0x40) && s.x_spd < -0x1c8 { return None; } // turned too early

    if (s.left_screen_edge_pos < 0x11 || s.left_screen_edge_pos >= 0x40) && s.x_pos < 0xd6000 { return Some(self.h.get_steps_until_x_pos_at_least(s, 0xd6000) + 58); }
    // Some(self.h.get_steps_until_x_pos_at_most(s, 0xd4cf0))
    Some(self.h.get_steps_until_x_pos_at_most(s, self.min_x_pos - 0x10))
  }
  fn is_goal_state(&self, _s: &State, emu_result: &EmuResult) -> bool {
    if let &EmuResult::StateChangeVerticalPipe(cx, cy) = emu_result {
      _s.x_pos < 0xd4c10 && _s.left_screen_edge_pos >= 0x11 && _s.left_screen_edge_pos < 0x40 && cx == 0xd4 && cy == 5
    } else { false }
  }
  fn track_metric(&mut self, s: &State) -> () {
    if s.left_screen_edge_pos >= 0x11 && s.left_screen_edge_pos < 0x40 && self.min_x_pos > s.x_pos {
      self.min_x_pos = s.x_pos;
      println!("new best min_x_pos: {:x}", self.min_x_pos);
    }
  }
  fn report_metrics(&self) -> () {
    println!("best min_x_pos so far: {:x}", self.min_x_pos);
  }
}


/// Part 4 speed-up in 8-4
#[allow(dead_code)]
pub struct W84Part4Speedup {
  max_x_pos: i32,
  h: XPosHeuristic,
}
impl Options for W84Part4Speedup {
  type CoinHandler = IgnoreCoins;
  type Platform = NTSC;
  type PlayerSize = Big;
  type Swim = Swimming;
  type PowerupHandler = NoPowerups;
  type RunningTimer = NoRunningTimer;
  type ScrollPos = NoScrollPos;
  type Parity = NoParity;
  type VerticalPipeHandler = IgnoreVerticalPipes;
  type YPosFractionalBehavior = KeepYPosFractionals;
}
impl super::SmbSearchCase for W84Part4Speedup {
  type SearchGoal = Self;
  type StateStore = ::store::VecHashMap<CompressedState<Self, [u8; 11]>, Dist>;

  type BlockBuffer = BB84Sub;

  fn start_states() -> Vec<State> {
    let s = State {
      x_pos: 0x3800,
      y_pos: 0x19000,
      x_spd: 0x0,
      y_spd: 0x0,
      player_state: PlayerState::STANDING,
      moving_dir: Dir::empty(),
      facing_dir: Dir::RIGHT,
      v_force: <Self as Options>::Platform::V_FORCE_AREA_INIT,
      v_force_down: <Self as Options>::Platform::V_FORCE_AREA_INIT,
      x_spd_abs: 0x0,
      running_speed: false,
      collision_bits: Dir::LR,
      is_crouching: false,
      jump_swim_timer: 0,
      running_timer : 0,
      left_screen_edge_pos: 0x0,
      side_collision_timer: 0,
      collected_coins: 0,
      powerup_block_hit: false,
      powerup_collected: false,
      parity: 0,
    };
    vec![s]
  }
  const SEARCH_SPACE_SIZE_HINT: usize = 80127;
}
impl SearchGoal for W84Part4Speedup {
  fn new() -> Self { return Self { max_x_pos: 0, h: XPosHeuristic::new::<Self>(&<Self as super::SmbSearchCase>::start_states()) }; }
  fn distance_to_goal_heuristic(&self, s: &mut State, _: Dist) -> Option<Dist> {
    // Some(self.h.get_steps_until_x_pos_at_least(s, 0x3d50))
    Some(self.h.get_steps_until_x_pos_at_least(s, self.max_x_pos + 0x10))
  }
  fn is_goal_state(&self, _s: &State, _: &EmuResult) -> bool {
    _s.x_pos >= 0xb950
    // false
  }
  fn track_metric(&mut self, s: &State) -> () {
    if self.max_x_pos < s.x_pos {
      self.max_x_pos = s.x_pos;
      println!("new best max_x_pos: {:x}", self.max_x_pos);
    }
  }
  fn report_metrics(&self) -> () {
    println!("best max_x_pos so far: {:x}", self.max_x_pos);
  }
}
