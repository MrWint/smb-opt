use blockbuffer::world7::*;
#[allow(unused_imports)] use emu::{Emu, EmuResult};
#[allow(unused_imports)] use emu::inputs::*;
use heuristics::SearchGoal;
use heuristics::xpos::XPosHeuristic;
use options::*;
use state::{CompressedState, Dir, Dist, PlayerState, State};
use std::cmp::max;


/// Clip through second 3-high launcher for screen scroll
// run through: 0x35c90
/// Input sequence: [1x A|R, 3x A, 8x R, 1x L, 1x R, 4x NIL, 1x R, 13x NIL, 1x L|R, 12x R, 2x L|R, 1x B|R, 1x L, 1x A|R, 14x R] (len: 64) - scroll 31; 0x31e60 (-0x3E30)
/// Input sequence: [1x A|R, 3x A, 8x R, 1x L, 1x R, 4x NIL, 1x R, 12x NIL, 1x D, 1x L|R, 12x R, 2x L|R, 2x B|R, 1x L, 1x A|R, 13x R] (len: 64) - scroll 32; 0x31da0 (-0x3EF0) [-0xc0]
/// Input sequence: [3x A, 1x A|R, 8x R, 1x L, 1x R, 4x NIL, 1x R, 12x NIL, 1x D, 1x L|R, 1x NIL, 2x L, 9x R, 6x L|R, 1x B|R, 1x L, 1x A|R, 10x R] (len: 64) - scroll 33; 0x316c0 (-0x45D0) [-0x6E0]
#[allow(dead_code)]
pub struct W71Launcher2Clip {
  max_x_pos: i32,
  h: XPosHeuristic,
}
impl Options for W71Launcher2Clip {
  type CoinHandler = IgnoreCoins;
  type Platform = NTSC;
  type PlayerSize = Big;
  type Swim = NotSwimming;
  type PowerupHandler = NoPowerups;
  type RunningTimer = NoRunningTimer;
  type ScrollPos = WithScrollPos;
  type Parity = NoParity;
  type VerticalPipeHandler = IgnoreVerticalPipes;
  type YPosFractionalBehavior = KeepYPosFractionals;
}
impl super::SmbSearchCase for W71Launcher2Clip {
  type SearchGoal = Self;
  type StateStore = ::store::VecHashMap<CompressedState<Self, [u8; 12]>, Dist>;

  type BlockBuffer = BB71;

  fn start_states() -> Vec<State> {
    let s = State {
      x_pos: 0x2bc70,
      y_pos: 0x1ab00,
      x_spd: 0x27ec,
      y_spd: -0x4d8, // 0xfb28,
      player_state: PlayerState::JUMPING,
      moving_dir: Dir::RIGHT,
      facing_dir: Dir::LEFT,
      v_force: <Self as Options>::Platform::V_FORCE_AREA_INIT,
      v_force_down: <Self as Options>::Platform::V_FORCE_FALL_RUNNING,
      x_spd_abs: 0x21,
      running_speed: true,
      collision_bits: Dir::LR,
      is_crouching: false,
      jump_swim_timer: 31,
      running_timer : 8,
      left_screen_edge_pos: 0x4c,
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
  const INITIAL_SEARCH_DISTANCE: Dist = 64;
  const SEARCH_SPACE_SIZE_HINT: usize = 800000000;
}
impl SearchGoal for W71Launcher2Clip {
  fn new() -> Self { return Self { max_x_pos: 0x2e000, h: XPosHeuristic::new::<Self>(&<Self as super::SmbSearchCase>::start_states()) }; }
  fn distance_to_goal_heuristic(&self, s: &mut State, _steps_already_taken: Dist) -> Option<Dist> {
    let rel_x_pos = ((s.x_pos >> 8) - s.left_screen_edge_pos as i32) & 0xff;

    const GOAL_SCROLL_POS: i32 = 0x92;
    if rel_x_pos > GOAL_SCROLL_POS { return None; }

    if s.x_pos >= 0x2da00 || s.side_collision_timer > 0 {
      if s.side_collision_timer < 15 {
        let max_x_pos = self.h.get_max_x_pos_after_steps(s, s.side_collision_timer as usize);
        let max_scroll = max((max_x_pos >> 8) - (s.x_pos >> 8), 0);
        if rel_x_pos + max_scroll < GOAL_SCROLL_POS { return None; }
      }
    }
    // let heuristic_steps = self.h.get_steps_until_x_pos_at_least(s, 0x31eb0);
    let heuristic_steps = self.h.get_steps_until_x_pos_at_least(s, self.max_x_pos + 0x10);
    Some(heuristic_steps + if s.x_pos < 0x2da00 && s.side_collision_timer == 0 { 15 } else { 0 }) // adjust for speed loss when colliding
  }
  fn is_goal_state(&self, _s: &State, _: &EmuResult) -> bool {
    _s.x_pos >= 0x316c0
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


/// Clip through second 3-high pipe for screen scroll
// run through: 0x665a0 (len 70)
// Input sequence: [6x R, 2x NIL, 1x L, 1x R, 4x NIL, 1x R, 29x NIL, 1x L|R, 12x R, 2x L|R, 1x B|R, 1x L, 1x A|R, 8x R] (len: 70) - scroll 47; 0x60f60 (-5640)
// Input sequence: [6x R, 2x NIL, 1x L, 1x R, 4x NIL, 1x R, 28x NIL, 1x D, 1x L|R, 12x R, 2x L|R, 2x B|R, 1x L, 1x A|R, 7x R] (len: 70) - scroll 48; 0x60ea0 (-5700) [-C0]
// Input sequence: [6x R, 1x L, 1x NIL, 1x L, 5x NIL, 1x R, 28x NIL, 1x D, 1x L|R, 1x NIL, 2x L, 9x R, 6x L|R, 1x B|R, 1x L, 1x A|R, 7x R] (len: 73) - scroll 49; 0x60f40 (-5DE0) [-6E0]

#[allow(dead_code)]
pub struct W71Pipe2Clip {
  max_x_pos: i32,
  h: XPosHeuristic,
}
impl Options for W71Pipe2Clip {
  type CoinHandler = IgnoreCoins;
  type Platform = NTSC;
  type PlayerSize = Big;
  type Swim = NotSwimming;
  type PowerupHandler = NoPowerups;
  type RunningTimer = NoRunningTimer;
  type ScrollPos = WithScrollPos;
  type Parity = NoParity;
  type VerticalPipeHandler = IgnoreVerticalPipes;
  type YPosFractionalBehavior = KeepYPosFractionals;
}
impl super::SmbSearchCase for W71Pipe2Clip {
  type SearchGoal = Self;
  type StateStore = ::store::VecHashMap<CompressedState<Self, [u8; 12]>, Dist>;

  type BlockBuffer = BB71;

  fn start_states() -> Vec<State> {
    let s = State {
      x_pos: 0x5ac70,
      y_pos: 0x1ab00,
      x_spd: 0x2758,
      y_spd: -0x4d8, // 0xfb28,
      player_state: PlayerState::JUMPING,
      moving_dir: Dir::RIGHT,
      facing_dir: Dir::LEFT,
      v_force: <Self as Options>::Platform::V_FORCE_AREA_INIT,
      v_force_down: <Self as Options>::Platform::V_FORCE_FALL_RUNNING,
      x_spd_abs: 0x21,
      running_speed: true,
      collision_bits: Dir::LR,
      is_crouching: false,
      jump_swim_timer: 31,
      running_timer : 8,
      left_screen_edge_pos: 0x1c,
      side_collision_timer: 0,
      collected_coins: 0,
      powerup_block_hit: false,
      powerup_collected: false,
      parity: 0,
    };
    // let s = Self::Emu::run_steps_nr(s, &[A|R; 1]);
    // let s = Self::Emu::run_steps_nr(s, &[A; 3]);
    // super::with_smaller_x_pos::<Self>(super::with_left_and_right_facing_dir(super::with_all_x_spd_subpixels(
    vec![s]
    // )), 16)
  }
  const INITIAL_SEARCH_DISTANCE: Dist = 74;
  const SEARCH_SPACE_SIZE_HINT: usize = 800000000;
}
impl SearchGoal for W71Pipe2Clip {
  fn new() -> Self { return Self { max_x_pos: 0x5d000, h: XPosHeuristic::new::<Self>(&<Self as super::SmbSearchCase>::start_states()) }; }
  fn distance_to_goal_heuristic(&self, s: &mut State, _steps_already_taken: Dist) -> Option<Dist> {
    let rel_x_pos = ((s.x_pos >> 8) - s.left_screen_edge_pos as i32) & 0xff;

    const GOAL_SCROLL_POS: i32 = 0x90 + 0x30;
    if rel_x_pos > GOAL_SCROLL_POS { return None; }

    if s.x_pos >= 0x5ca00 || s.side_collision_timer > 0 {
      if s.side_collision_timer < 15 {
        let max_x_pos = self.h.get_max_x_pos_after_steps(s, s.side_collision_timer as usize);
        let max_scroll = max((max_x_pos >> 8) - (s.x_pos >> 8), 0);
        if rel_x_pos + max_scroll < GOAL_SCROLL_POS { return None; }
      }
    }
    // let heuristic_steps = self.h.get_steps_until_x_pos_at_least(s, 0x31eb0);
    let heuristic_steps = self.h.get_steps_until_x_pos_at_least(s, self.max_x_pos + 0x10);
    Some(heuristic_steps + if s.x_pos < 0x5ca00 && s.side_collision_timer == 0 { 15 } else { 0 }) // adjust for speed loss when colliding
  }
  fn is_goal_state(&self, _s: &State, _: &EmuResult) -> bool {
    // _s.x_pos >= 0x60f60
    false
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


/// Clip through first 2-high pipe for screen scroll
/// run through: 0x7c760
/// Input sequence: [4x R, 1x L, 1x NIL, 3x L, 33x NIL, 1x D, 1x L|R, 8x R, 6x L|R, 2x B|R, 1x L, 1x A|R, 8x R] (len: 70) - scroll 48; 0x771a0

#[allow(dead_code)]
pub struct W71Pipe1Clip {
  max_x_pos: i32,
  h: XPosHeuristic,
}
impl Options for W71Pipe1Clip {
  type CoinHandler = IgnoreCoins;
  type Platform = NTSC;
  type PlayerSize = Big;
  type Swim = NotSwimming;
  type PowerupHandler = NoPowerups;
  type RunningTimer = NoRunningTimer;
  type ScrollPos = WithScrollPos;
  type Parity = NoParity;
  type VerticalPipeHandler = IgnoreVerticalPipes;
  type YPosFractionalBehavior = KeepYPosFractionals;
}
impl super::SmbSearchCase for W71Pipe1Clip {
  type SearchGoal = Self;
  type StateStore = ::store::VecHashMap<CompressedState<Self, [u8; 12]>, Dist>;

  type BlockBuffer = BB71;

  fn start_states() -> Vec<State> {
    let s = State {
      x_pos: 0x71860,
      y_pos: 0x1a558,
      x_spd: 0x28a0,
      y_spd: -0x400, // 0xfc00,
      player_state: PlayerState::JUMPING,
      moving_dir: Dir::RIGHT,
      facing_dir: Dir::RIGHT,
      v_force: <Self as Options>::Platform::V_FORCE_FALL_RUNNING,
      v_force_down: <Self as Options>::Platform::V_FORCE_FALL_RUNNING,
      x_spd_abs: 0x21,
      running_speed: true,
      collision_bits: Dir::LR,
      is_crouching: false,
      jump_swim_timer: 0,
      running_timer : 0,
      left_screen_edge_pos: 0x59,
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
  const INITIAL_SEARCH_DISTANCE: Dist = 70;
  const SEARCH_SPACE_SIZE_HINT: usize = 800000000;
}
impl SearchGoal for W71Pipe1Clip {
  fn new() -> Self { return Self { max_x_pos: 0x73000, h: XPosHeuristic::new::<Self>(&<Self as super::SmbSearchCase>::start_states()) }; }
  fn distance_to_goal_heuristic(&self, s: &mut State, _steps_already_taken: Dist) -> Option<Dist> {
    let rel_x_pos = ((s.x_pos >> 8) - s.left_screen_edge_pos as i32) & 0xff;

    const GOAL_SCROLL_POS: i32 = 0xbf + 0x30;
    if rel_x_pos > GOAL_SCROLL_POS { return None; }

    if s.x_pos >= 0x72a00 || s.side_collision_timer > 0 {
      if s.side_collision_timer < 15 {
        let max_x_pos = self.h.get_max_x_pos_after_steps(s, s.side_collision_timer as usize);
        let max_scroll = max((max_x_pos >> 8) - (s.x_pos >> 8), 0);
        if rel_x_pos + max_scroll < GOAL_SCROLL_POS { return None; }
      }
    }
    // let heuristic_steps = self.h.get_steps_until_x_pos_at_least(s, 0x31eb0);
    let heuristic_steps = self.h.get_steps_until_x_pos_at_least(s, self.max_x_pos + 0x10);
    Some(heuristic_steps + if s.x_pos < 0x72a00 && s.side_collision_timer == 0 { 15 } else { 0 }) // adjust for speed loss when colliding
  }
  fn is_goal_state(&self, _s: &State, _: &EmuResult) -> bool {
    _s.x_pos >= 0x771a0
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
