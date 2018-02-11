use blockbuffer::world2::*;
#[allow(unused_imports)] use emu::{Emu, EmuResult};
#[allow(unused_imports)] use emu::inputs::*;
use heuristics::SearchGoal;
use heuristics::xpos::XPosHeuristic;
use options::*;
use state::{CompressedState, Dir, Dist, PlayerState, State};
use std::cmp::max;


/// Scroll screen by collision
/// Input sequence: [3x B|R, 1x NIL, 1x L, 1x A, 6x R, 7x NIL, 7x R, 7x L|R, 1x B|R, 1x L, 1x A|R, 9x R] (len: 45) x_pos loss: 0x2100 (~13 frames), scroll 11
/// Input sequence: [1x B|R, 1x NIL, 1x B|R, 1x L, 4x A, 10x A|R, 1x A, 9x R, 5x L|R, 1x B|R, 1x L, 1x A|R, 9x R] (len: 45) x_pos loss: 0x20e0 (~13 frames), scroll 10
#[allow(dead_code)]
pub struct W21ScreenScrollRightCollision {
  max_x_pos: i32,
  h: XPosHeuristic,
}
impl Options for W21ScreenScrollRightCollision {
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
impl super::SmbSearchCase for W21ScreenScrollRightCollision {
  type SearchGoal = Self;
  type StateStore = ::store::VecHashMap<CompressedState<Self, [u8; 12]>, Dist>;

  type BlockBuffer = BB21;

  fn start_states() -> Vec<State> {
    let s = State {
      x_pos: 0xb710,
      y_pos: 0x1b028,
      x_spd: 0x2840,
      y_spd: 0x0,
      player_state: PlayerState::STANDING,
      moving_dir: Dir::RIGHT,
      facing_dir: Dir::RIGHT,
      v_force: <Self as Options>::Platform::V_FORCE_FALL_RUNNING,
      v_force_down: <Self as Options>::Platform::V_FORCE_FALL_RUNNING,
      x_spd_abs: 0x21,
      running_speed: true,
      collision_bits: Dir::LR,
      is_crouching: false,
      jump_swim_timer: 0,
      running_timer : 10,
      left_screen_edge_pos: 0x47,
      side_collision_timer: 0,
      coin_collected: false,
      powerup_block_hit: false,
      powerup_collected: false,
      parity: 0,
    };
    super::with_smaller_x_pos::<Self>(super::with_left_and_right_facing_dir(super::with_all_x_spd_subpixels(
    vec![s]
    )), 0)
  }
  const INITIAL_SEARCH_DISTANCE: Dist = 45;
  const SEARCH_SPACE_SIZE_HINT: usize = 800000000;
}
impl SearchGoal for W21ScreenScrollRightCollision {
  fn new() -> Self { return Self { max_x_pos: 0x10680, h: XPosHeuristic::new::<Self>(&<Self as super::SmbSearchCase>::start_states()) }; }
  fn distance_to_goal_heuristic(&self, s: &mut State, _steps_already_taken: Dist) -> Option<Dist> {
    let rel_x_pos = ((s.x_pos >> 8) - s.left_screen_edge_pos as i32) & 0xff;

    const GOAL_SCROLL_POS: i32 = 0x7a;

    if s.x_pos >= 0xe300 || s.side_collision_timer > 0 {
      if s.side_collision_timer < 14 {
        let max_x_pos = self.h.get_max_x_pos_after_steps(s, s.side_collision_timer as usize);
        let max_scroll = max((max_x_pos >> 8) - (s.x_pos >> 8), 0);
        if rel_x_pos + max_scroll < GOAL_SCROLL_POS { return None; }
      }
    }
    let heuristic_steps = self.h.get_steps_until_x_pos_at_least(s, 0x106b0);
    // let heuristic_steps = self.h.get_steps_until_x_pos_at_least(s, self.max_x_pos + 0x10);
    Some(heuristic_steps + if s.x_pos < 0xe080 { 10 } else { 0 }) // adjust for speed loss when colliding
  }
  fn is_goal_state(&self, s: &State, _: &EmuResult) -> bool {
    s.x_pos >= 0x106b0
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


/// Scroll screen by collision
/// Input sequence: [1x B|R, 1x L, 1x B|R, 1x L, 2x B|R, 1x R, 1x L, 5x NIL, 2x A|L, 7x L, 14x R, 2x B|R, 1x L, 1x A|R, 5x R] (len: 45) x_pos loss: 0x3870 (~22.6 frames), scroll 14

#[allow(dead_code)]
pub struct W21ScreenScrollLeftCollision {
  max_x_pos: i32,
  h: XPosHeuristic,
}
impl Options for W21ScreenScrollLeftCollision {
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
impl super::SmbSearchCase for W21ScreenScrollLeftCollision {
  type SearchGoal = Self;
  type StateStore = ::store::VecHashMap<CompressedState<Self, [u8; 12]>, Dist>;

  type BlockBuffer = BB21;

  fn start_states() -> Vec<State> {
    let s = State {
      x_pos: 0x10210,
      y_pos: 0x1b028,
      x_spd: 0x28f8,
      y_spd: 0x0,
      player_state: PlayerState::STANDING,
      moving_dir: Dir::RIGHT,
      facing_dir: Dir::RIGHT,
      v_force: <Self as Options>::Platform::V_FORCE_FALL_RUNNING,
      v_force_down: <Self as Options>::Platform::V_FORCE_FALL_RUNNING,
      x_spd_abs: 0x21,
      running_speed: true,
      collision_bits: Dir::LR,
      is_crouching: false,
      jump_swim_timer: 0,
      running_timer : 10,
      left_screen_edge_pos: 0x92,
      side_collision_timer: 0,
      coin_collected: false,
      powerup_block_hit: false,
      powerup_collected: false,
      parity: 0,
    };
    // super::with_smaller_x_pos::<Self>(super::with_left_and_right_facing_dir(super::with_all_x_spd_subpixels(
    vec![s]
    // )), 0)
  }
  const INITIAL_SEARCH_DISTANCE: Dist = 45;
  const SEARCH_SPACE_SIZE_HINT: usize = 800000000;
}
impl SearchGoal for W21ScreenScrollLeftCollision {
  fn new() -> Self { return Self { max_x_pos: 0x13440, h: XPosHeuristic::new::<Self>(&<Self as super::SmbSearchCase>::start_states()) }; }
  fn distance_to_goal_heuristic(&self, s: &mut State, _steps_already_taken: Dist) -> Option<Dist> {
    let rel_x_pos = ((s.x_pos >> 8) - s.left_screen_edge_pos as i32) & 0xff;

    const GOAL_SCROLL_POS: i32 = 0x7f;

    if s.x_pos >= 0x11f00 || s.side_collision_timer > 0 {
      let max_x_pos = self.h.get_max_x_pos_after_steps(s, s.side_collision_timer as usize);
      let max_scroll = max((max_x_pos >> 8) - (s.x_pos >> 8), 0);
      if rel_x_pos + max_scroll < GOAL_SCROLL_POS { return None; }
    }
    // let heuristic_steps = self.h.get_steps_until_x_pos_at_least(s, 0x13a40);
    let heuristic_steps = self.h.get_steps_until_x_pos_at_least(s, self.max_x_pos + 0x10);
    Some(heuristic_steps)
  }
  fn is_goal_state(&self, s: &State, _: &EmuResult) -> bool {
    // s.x_pos >= 0x13a40
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


/// Scroll screen by clipping through 3-high blocks
/// Input sequence: [2x B|R, 1x L, 1x B|R, 1x L, 1x A, 1x A|R, 9x A, 1x R, 5x NIL, 1x L, 5x NIL, 1x R, 1x A|L, 16x R, 1x B|R, 8x R] (len: 55) x_pos loss: 0x3020 (~19.2 frames), scroll 18
#[allow(dead_code)]
pub struct W21ScreenScrollBlock3Clip {
  max_x_pos: i32,
  h: XPosHeuristic,
}
impl Options for W21ScreenScrollBlock3Clip {
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
impl super::SmbSearchCase for W21ScreenScrollBlock3Clip {
  type SearchGoal = Self;
  type StateStore = ::store::VecHashMap<CompressedState<Self, [u8; 12]>, Dist>;

  type BlockBuffer = BB21;

  fn start_states() -> Vec<State> {
    let s = State {
      x_pos: 0x1e310,
      y_pos: 0x1b040,
      x_spd: 0x2820,
      y_spd: 0x0,
      player_state: PlayerState::STANDING,
      moving_dir: Dir::RIGHT,
      facing_dir: Dir::RIGHT,
      v_force: <Self as Options>::Platform::V_FORCE_FALL_RUNNING,
      v_force_down: <Self as Options>::Platform::V_FORCE_FALL_RUNNING,
      x_spd_abs: 0x21,
      running_speed: true,
      collision_bits: Dir::LR,
      is_crouching: false,
      jump_swim_timer: 0,
      running_timer : 10,
      left_screen_edge_pos: 0x73,
      side_collision_timer: 0,
      coin_collected: false,
      powerup_block_hit: false,
      powerup_collected: false,
      parity: 0,
    };
    super::with_smaller_x_pos::<Self>(super::with_left_and_right_facing_dir(super::with_all_x_spd_subpixels(
    vec![s]
    )), 0)
  }
  const INITIAL_SEARCH_DISTANCE: Dist = 55;
  const SEARCH_SPACE_SIZE_HINT: usize = 800000000;
}
impl SearchGoal for W21ScreenScrollBlock3Clip {
  fn new() -> Self { return Self { max_x_pos: 0x23900, h: XPosHeuristic::new::<Self>(&<Self as super::SmbSearchCase>::start_states()) }; }
  fn distance_to_goal_heuristic(&self, s: &mut State, _steps_already_taken: Dist) -> Option<Dist> {
    let rel_x_pos = ((s.x_pos >> 8) - s.left_screen_edge_pos as i32) & 0xff;

    const GOAL_SCROLL_POS: i32 = 0x81;

    if s.x_pos >= 0x21a00 || s.side_collision_timer > 0 {
      if s.side_collision_timer < 15 {
        let max_x_pos = self.h.get_max_x_pos_after_steps(s, s.side_collision_timer as usize);
        let max_scroll = max((max_x_pos >> 8) - (s.x_pos >> 8), 0);
        if rel_x_pos + max_scroll < GOAL_SCROLL_POS { return None; }
      }
    }
    let heuristic_steps = self.h.get_steps_until_x_pos_at_least(s, 0x23dc0);
    // let heuristic_steps = self.h.get_steps_until_x_pos_at_least(s, self.max_x_pos + 0x10);
    Some(heuristic_steps + if s.x_pos < 0x21a00 && s.side_collision_timer == 0 { 15 } else { 0 }) // adjust for speed loss when colliding
  }
  fn is_goal_state(&self, s: &State, _: &EmuResult) -> bool {
    s.x_pos >= 0x23dc0
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


/// Scroll screen by clipping through 3-high blocks
/// Input sequence: [1x L, 5x NIL, 1x R, 1x A, 1x L, 15x R, 1x B|R, 1x L, 14x R] (len: 40) x_pos loss: 0x2ff0 (~19.2 frames), scroll 18
#[allow(dead_code)]
pub struct W21ScreenScrollBlock3ClipSpeedup {
  max_x_pos: i32,
  h: XPosHeuristic,
}
impl Options for W21ScreenScrollBlock3ClipSpeedup {
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
impl super::SmbSearchCase for W21ScreenScrollBlock3ClipSpeedup {
  type SearchGoal = Self;
  type StateStore = ::store::VecHashMap<CompressedState<Self, [u8; 12]>, Dist>;

  type BlockBuffer = BB21;

  fn start_states() -> Vec<State> {
    let s = State {
      x_pos: 0x218e0,
      y_pos: 0x17658,
      x_spd: 0x64,
      y_spd: 0x18,
      player_state: PlayerState::JUMPING,
      moving_dir: Dir::RIGHT,
      facing_dir: Dir::LEFT,
      v_force: <Self as Options>::Platform::V_FORCE_FALL_RUNNING,
      v_force_down: <Self as Options>::Platform::V_FORCE_FALL_RUNNING,
      x_spd_abs: 0x21,
      running_speed: true,
      collision_bits: Dir::LEFT,
      is_crouching: false,
      jump_swim_timer: 15,
      running_timer : 0,
      left_screen_edge_pos: 0xa9,
      side_collision_timer: 15,
      coin_collected: false,
      powerup_block_hit: false,
      powerup_collected: false,
      parity: 0,
    };
    super::with_smaller_x_pos::<Self>(super::with_left_and_right_facing_dir(super::with_all_x_spd_subpixels(
    vec![s]
    )), 16)
  }
  const INITIAL_SEARCH_DISTANCE: Dist = 40;
  const SEARCH_SPACE_SIZE_HINT: usize = 800000000;
}
impl SearchGoal for W21ScreenScrollBlock3ClipSpeedup {
  fn new() -> Self { return Self { max_x_pos: 0x24de0, h: XPosHeuristic::new::<Self>(&<Self as super::SmbSearchCase>::start_states()) }; }
  fn distance_to_goal_heuristic(&self, s: &mut State, _steps_already_taken: Dist) -> Option<Dist> {
    let rel_x_pos = ((s.x_pos >> 8) - s.left_screen_edge_pos as i32) & 0xff;

    const GOAL_SCROLL_POS: i32 = 0x81;

    if s.x_pos >= 0x21a00 || s.side_collision_timer > 0 {
      if s.side_collision_timer < 15 {
        let max_x_pos = self.h.get_max_x_pos_after_steps(s, s.side_collision_timer as usize);
        let max_scroll = max((max_x_pos >> 8) - (s.x_pos >> 8), 0);
        if rel_x_pos + max_scroll < GOAL_SCROLL_POS { return None; }
      }
    }
    let heuristic_steps = self.h.get_steps_until_x_pos_at_least(s, 0x24e20);
    // let heuristic_steps = self.h.get_steps_until_x_pos_at_least(s, self.max_x_pos + 0x10);
    Some(heuristic_steps)
  }
  fn is_goal_state(&self, s: &State, _: &EmuResult) -> bool {
    s.x_pos >= 0x24e20
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


/// Scroll screen by clipping through 3-high pipe
/// Input sequence: [10x A, 1x A|R, 5x R, 2x L, 1x R, 4x NIL, 1x R, 1x A|R, 15x R, 1x NIL, 1x B|R, 1x L, 1x A|R, 11x R] (len: 55) 0x4cd30, x_pos loss: 0x2be0 (~17.5 frames), scroll 17
/// Input sequence: [10x A, 1x A|R, 5x R, 2x L, 1x R, 5x NIL, 1x A|R, 16x R, 1x B|R, 1x L, 1x A|R, 11x R] (len: 55) 0x4ccb0, x_pos loss: 0x2c60 (XXX frames), scroll 18 (-0x80)
/// Input sequence: [11x A, 3x R, 2x NIL, 2x L, 6x NIL, 1x A, 16x R, 1x B|R, 1x L, 1x A|R, 11x R] (len: 55), 0x4cc10 x_pos loss: 0x2d00 (XXX frames), scroll 19 (-0xa0)
/// Input sequence: [11x A, 2x R, 3x NIL, 2x L, 1x R, 5x NIL, 1x L|R, 1x A|R, 15x R, 1x NIL, 1x B|R, 1x L, 1x A|R, 10x R] (len: 55) 0x4cbd0, x_pos loss: 0x2d40 (XXX frames), scroll 20 (-0x40)
/// Input sequence: [11x A, 2x R, 3x NIL, 2x L, 1x R, 5x NIL, 1x L|R, 1x A, 16x R, 1x B|R, 1x L, 1x A|R, 10x R] (len: 55) 0x4cb50, x_pos loss: 0x2dc0 (XXX frames), scroll 21 (-0x80)
/// Input sequence: [11x A, 2x R, 3x NIL, 2x L, 1x R, 5x NIL, 1x L|R, 1x A, 1x NIL, 15x R, 1x B|R, 1x L, 1x A|R, 10x R] (len: 55) 0x4ca60, x_pos loss: 0x2eb0 (~18.7 frames), scroll 22 (-0xf0)
/// Input sequence: [11x A, 2x R, 3x NIL, 2x L, 1x R, 5x NIL, 1x L|R, 1x NIL, 1x A, 1x NIL, 15x R, 1x B|R, 1x L, 1x A|R, 9x R] (len: 55) 0x4c8e0, x_pos loss: 0x3030 (XXX frames), scroll 23 (-0x180)
/// Input sequence: [11x A, 2x R, 3x NIL, 2x L, 1x R, 5x NIL, 1x R, 1x D, 1x L|R, 1x A, 1x NIL, 15x R, 1x B|R, 1x L, 1x A|R, 8x R] (len: 55) 0x4c760, x_pos loss: 0x31b0 (XXX frames), scroll 24 (-0x180)
/// Input sequence: [11x A, 2x R, 3x NIL, 2x L, 1x R, 5x NIL, 1x R, 1x NIL, 1x D, 1x L|R, 1x A, 1x NIL, 15x R, 1x B|R, 1x L, 1x A|R, 7x R] (len: 55) 0x4c5e0, x_pos loss: 0x3330 (XXX frames), scroll 25 (-0x180)
/// Input sequence: [11x A, 2x R, 3x NIL, 2x L, 1x R, 5x NIL, 1x R, 2x NIL, 1x D, 1x L|R, 1x A, 1x NIL, 15x R, 1x B|R, 1x L, 1x A|R, 7x R] (len: 56) 0x4c6e0, x_pos loss: 0x34b0 (XXX frames), scroll 26 (-0x180)
/// Input sequence: [11x A, 2x R, 3x NIL, 2x L, 1x R, 5x NIL, 1x R, 3x NIL, 1x D, 1x L|R, 1x A, 1x NIL, 15x R, 1x B|R, 1x L, 1x A|R, 7x R] (len: 57) 0x4c7e0, x_pos loss: 0x3630 (XXX frames), scroll 27 (-0x180)
/// Input sequence: [11x A, 2x R, 3x NIL, 2x L, 1x R, 5x NIL, 1x R, 4x NIL, 1x D, 1x L|R, 1x A, 1x NIL, 15x R, 1x B|R, 1x L, 1x A|R, 7x R] (len: 58) 0x4c8e0, x_pos loss: 0x37b0 (XXX frames), scroll 28 (-0x180)
/// Input sequence: [11x A, 2x R, 3x NIL, 2x L, 1x R, 5x NIL, 1x R, 5x NIL, 1x D, 1x L|R, 1x A, 1x NIL, 15x R, 1x B|R, 1x L, 1x A|R, 7x R] (len: 59) 0x4c9e0, x_pos loss: 0x3930 (XXX frames), scroll 29 (-0x180)
/// Input sequence: [11x A, 2x R, 3x NIL, 2x L, 1x R, 5x NIL, 1x R, 6x NIL, 1x D, 1x L|R, 1x A, 1x NIL, 15x R, 1x B|R, 1x L, 1x A|R, 7x R] (len: 60) 0x4cae0, x_pos loss: 0x3ab0 (XXX frames), scroll 30 (-0x180)
/// Input sequence: [11x A, 2x R, 3x NIL, 2x L, 1x R, 5x NIL, 1x R, 7x NIL, 1x D, 1x L|R, 1x A, 1x NIL, 15x R, 1x B|R, 1x L, 1x A|R, 7x R] (len: 61) 0x4cbe0, x_pos loss: 0x3c30 (XXX frames), scroll 31 (-0x180)
/// Input sequence: [11x A, 2x R, 3x NIL, 2x L, 1x R, 5x NIL, 1x R, 8x NIL, 1x D, 1x L|R, 1x A, 1x NIL, 15x R, 1x B|R, 1x L, 1x A|R, 7x R] (len: 62) 0x4cce0, x_pos loss: 0x3db0 (XXX frames), scroll 32 (-0x180)
/// Input sequence: [11x A, 2x R, 3x NIL, 2x L, 1x R, 5x NIL, 1x R, 9x NIL, 1x D, 1x L|R, 1x A, 1x NIL, 15x R, 1x B|R, 1x L, 8x R] (len: 63) 0x4cde0, x_pos loss: 0x3f30 (XXX frames), scroll 33 (-0x180)
/// Input sequence: [11x A, 2x R, 3x NIL, 2x L, 1x R, 5x NIL, 1x R, 10x NIL, 1x D, 1x L|R, 1x A, 1x NIL, 15x R, 1x B|R, 1x L, 8x R] (len: 64) 0x4cee0, x_pos loss: 0x40b0 (25.9 frames), scroll 34 (-0x180)
/// Input sequence: [11x A, 2x R, 3x NIL, 2x L, 1x R, 5x NIL, 1x R, 11x NIL, 1x D, 1x L|R, 1x A, 1x NIL, 15x R, 1x B|R, 15x R] (len: 71) 0x4dc60, x_pos loss: 0x44b0 (XXX frames), scroll 35 (-0x400)
/// Input sequence: [11x A, 2x R, 3x NIL, 2x L, 1x R, 5x NIL, 1x R, 12x NIL, 1x D, 1x L|R, 1x A, 1x NIL, 29x R, 2x NIL, 3x B|R, 1x L, 1x A|R, 5x R] (len: 82) 0x4e8b0, x_pos loss: 0x53e0 (XXX frames), scroll 36 (-0xf30)
/// Input sequence: [11x A, 2x R, 3x NIL, 2x L, 1x R, 5x NIL, 1x R, 13x NIL, 1x D, 1x L|R, 1x A, 1x NIL, 29x R, 2x NIL, 3x B|R, 1x L, 1x A|R, 5x R] (len: 83) 0x4e9b0, x_pos loss: 0x5560 (XXX frames), scroll 37 (-0x180)
/// Input sequence: [11x A, 2x R, 3x NIL, 2x L, 1x R, 5x NIL, 1x R, 14x NIL, 1x D, 1x L|R, 1x A, 1x NIL, 29x R, 2x NIL, 2x B|R, 1x L, 1x A|R, 6x R] (len: 84) 0x4eb00, x_pos loss: 0x5690 (XXX frames), scroll 38 (-0x130)
/// Input sequence: [11x A, 2x R, 3x NIL, 2x L, 1x R, 5x NIL, 1x R, 15x NIL, 1x D, 1x L|R, 1x A, 1x NIL, 29x R, 2x NIL, 2x B|R, 1x L, 1x A|R, 6x R] (len: 85) 0x4ec00, x_pos loss: 0x5810 (XXX frames), scroll 39 (-0x180)
/// Input sequence: [11x A, 5x R, 1x NIL, 1x L, 1x R, 5x NIL, 1x R, 27x NIL, 1x D, 1x L|R, 16x R, 1x B|R, 1x L, 1x A | DOWN, 13x R] (len: 86) 0x4ee10, x_pos loss: 0x5880 (XXX frames), scroll 48 (-0x70)
/// Input sequence: [11x A, 6x R, 1x L, 6x NIL, 1x R, 28x NIL, 1x D, 1x L|R, 16x R, 1x B|R, 1x L, 1x A | DOWN, 12x R] (len: 86) 0x4ec80, x_pos loss: 0x5a10 (~36 frames), scroll 49 (-0x190)
/// Input sequence: [11x A, 2x R, 3x NIL, 2x L, 1x R, 5x NIL, 1x R, 28x NIL, 1x D, 1x L|R, 16x R, 1x B|R, 1x L, 1x A | DOWN, 12x R] (len: 86) 0x4ec70, x_pos loss: 0x5a20 (~36 frames), scroll 50 (-0x10)
#[allow(dead_code)]
pub struct W21ScreenScrollBlock3PipeClip {
  max_x_pos: i32,
  h: XPosHeuristic,
}
impl Options for W21ScreenScrollBlock3PipeClip {
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
impl super::SmbSearchCase for W21ScreenScrollBlock3PipeClip {
  type SearchGoal = Self;
  type StateStore = ::store::VecHashMap<CompressedState<Self, [u8; 12]>, Dist>;

  type BlockBuffer = BB21;

  fn start_states() -> Vec<State> {
    let s = State {
      x_pos: 0x46f90,
      y_pos: 0x1b048,
      x_spd: 0x2800,
      y_spd: 0x0,
      player_state: PlayerState::STANDING,
      moving_dir: Dir::RIGHT,
      facing_dir: Dir::LEFT,
      v_force: <Self as Options>::Platform::V_FORCE_FALL_RUNNING,
      v_force_down: <Self as Options>::Platform::V_FORCE_FALL_RUNNING,
      x_spd_abs: 0x21,
      running_speed: true,
      collision_bits: Dir::LR,
      is_crouching: false,
      jump_swim_timer: 0,
      running_timer : 10,
      left_screen_edge_pos: 0xff,
      side_collision_timer: 0,
      coin_collected: false,
      powerup_block_hit: false,
      powerup_collected: false,
      parity: 0,
    };
    let s = Self::Emu::run_steps_nr(s, &[A; 11-5]);
    super::with_smaller_x_pos::<Self>(super::with_left_and_right_facing_dir(super::with_all_x_spd_subpixels(
    vec![s]
    )), 0)
  }
  const INITIAL_SEARCH_DISTANCE: Dist = 55 +31-11+5;
  const SEARCH_SPACE_SIZE_HINT: usize = 800000000;
}
impl SearchGoal for W21ScreenScrollBlock3PipeClip {
  fn new() -> Self { return Self { max_x_pos: 0x4c000, h: XPosHeuristic::new::<Self>(&<Self as super::SmbSearchCase>::start_states()) }; }
  fn distance_to_goal_heuristic(&self, s: &mut State, _steps_already_taken: Dist) -> Option<Dist> {
    let rel_x_pos = ((s.x_pos >> 8) - s.left_screen_edge_pos as i32) & 0xff;

    const GOAL_SCROLL_POS: i32 = 0x81 + 34;

    if s.x_pos >= 0x49a00 || s.side_collision_timer > 0 {
      if s.side_collision_timer < 15 {
        let max_x_pos = self.h.get_max_x_pos_after_steps(s, s.side_collision_timer as usize);
        let max_scroll = max((max_x_pos >> 8) - (s.x_pos >> 8), 0);
        if rel_x_pos + max_scroll < GOAL_SCROLL_POS { return None; }
      }
    }
    // let heuristic_steps = self.h.get_steps_until_x_pos_at_least(s, 0x4ec70); //0x4cd30
    let heuristic_steps = self.h.get_steps_until_x_pos_at_least(s, self.max_x_pos + 0x10);
    Some(heuristic_steps + if s.x_pos < 0x49a00 && s.side_collision_timer == 0 { 15 } else { 0 }) // adjust for speed loss when colliding
  }
  fn is_goal_state(&self, s: &State, _: &EmuResult) -> bool {
    // s.x_pos >= 0x4ec70 //0x4cd30
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


/// Floor clip using Green Parakoopa in 2-1 as big Mario
/// Input sequence: [5x R, 3x L, 1x NIL, 1x L, 1x NIL, 1x L, 5x R, 1x L, 1x R, 5x NIL, 1x R, 1x A|L, 1x R, 1x NIL, 1x R, 1x A|L, 1x R, 1x NIL, 1x R, 10x B|R, 2x L|R, 1x B|R, 1x L|R, 1x B|R, 2x L|R, 1x B|R, 1x L|R, 1x B|R, 1x L|R, 16x B|R] (len: 70)
#[allow(dead_code)]
pub struct W21FloorClip {
  max_x_pos: i32,
  h: XPosHeuristic,
}
impl Options for W21FloorClip {
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
impl super::SmbSearchCase for W21FloorClip {
  type SearchGoal = Self;
  type StateStore = ::store::VecHashMap<CompressedState<Self, [u8; 10]>, Dist>;

  type BlockBuffer = BB21;

  fn start_states() -> Vec<State> {
    let s = State {
      x_pos: 0x96e30,
      y_pos: 0x19940,
      x_spd: 0x2890,
      y_spd: 0x400,
      player_state: PlayerState::JUMPING,
      moving_dir: Dir::RIGHT,
      facing_dir: Dir::RIGHT,
      v_force: <Self as Options>::Platform::V_FORCE_FALL_RUNNING,
      v_force_down: <Self as Options>::Platform::V_FORCE_FALL_RUNNING,
      x_spd_abs: 0x21,
      running_speed: true,
      collision_bits: Dir::LR,
      is_crouching: true,
      jump_swim_timer: 0,
      running_timer : 0,
      left_screen_edge_pos: 0xed,
      side_collision_timer: 0,
      coin_collected: false,
      powerup_block_hit: false,
      powerup_collected: false,
      parity: 0,
    };
    super::with_smaller_x_pos::<Self>(super::with_left_and_right_facing_dir(super::with_all_x_spd_subpixels(
    vec![s]
    )), 3)
  }
  const INITIAL_SEARCH_DISTANCE: Dist = 70;
  const SEARCH_SPACE_SIZE_HINT: usize = 800000000;
}
impl SearchGoal for W21FloorClip {
  fn new() -> Self { return Self { max_x_pos: 0x9cf20, h: XPosHeuristic::new::<Self>(&<Self as super::SmbSearchCase>::start_states()) }; }
  fn distance_to_goal_heuristic(&self, s: &mut State, _steps_already_taken: Dist) -> Option<Dist> {
    // Corresponds to Koopa position 0x80bxx
    if _steps_already_taken == 11 && s.y_pos >= 0x1c500 { // Koopa collision
      if s.y_spd < 0x100 { return None; } // Injured by Koopa
      s.y_spd = -0x400 + (s.y_spd & 0xff);
    }
    if s.x_pos >= 0x98800 && s.y_pos < 0x1b500 { return None; } // no floor clip

    Some(self.h.get_steps_until_x_pos_at_least(s, 0x9cf30))
    // Some(self.h.get_steps_until_x_pos_at_least(s, self.max_x_pos + 0x10))
  }
  fn is_goal_state(&self, s: &State, _: &EmuResult) -> bool {
    s.x_pos >= 0x9cf30 && s.y_pos >= 0x1c000
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




/// Full flag pole glitch in 1-3 from floor clip
/// Input sequence: [10x B|R, 2x A, 1x L, 2x NIL, 1x R] (len: 16)
#[allow(dead_code)]
pub struct W21FloorFlag {
  h: XPosHeuristic,
}
impl Options for W21FloorFlag {
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
impl super::SmbSearchCase for W21FloorFlag {
  type SearchGoal = Self;
  type StateStore = ::store::VecHashMap<CompressedState<Self, [u8; 10]>, Dist>;

  type BlockBuffer = BB21;

  fn start_states() -> Vec<State> {
    let s = State {
      x_pos: 0xc5930,
      y_pos: 0x1c0b0,
      x_spd: 0x28bc,
      y_spd: 0x0,
      player_state: PlayerState::STANDING,
      moving_dir: Dir::RIGHT,
      facing_dir: Dir::RIGHT,
      v_force: <Self as Options>::Platform::V_FORCE_FALL_STANDING,
      v_force_down: <Self as Options>::Platform::V_FORCE_FALL_STANDING,
      x_spd_abs: 0x21,
      running_speed: true,
      collision_bits: Dir::RIGHT,
      is_crouching: false,
      jump_swim_timer: 0,
      running_timer : 10,
      left_screen_edge_pos: 0xc7,
      side_collision_timer: 0,
      coin_collected: false,
      powerup_block_hit: false,
      powerup_collected: false,
      parity: 0,
    };
    // super::with_smaller_x_pos::<Self>(super::with_left_and_right_and_lr_facing_dir(super::with_all_x_spd_subpixels(
    vec![s]
    // )), 16)
  }
  const INITIAL_SEARCH_DISTANCE: Dist = 0;
  const SEARCH_SPACE_SIZE_HINT: usize = 10;
}
impl SearchGoal for W21FloorFlag {
  fn new() -> Self { return Self { h: XPosHeuristic::new::<Self>(&<Self as super::SmbSearchCase>::start_states()) }; }
  fn distance_to_goal_heuristic(&self, s: &mut State, _steps_already_taken: Dist) -> Option<Dist> {
    Some(self.h.get_steps_until_x_pos_at_least(s, 0xc7600))
  }
  fn is_goal_state(&self, s: &State, emu_result: &EmuResult) -> bool {
    if let &EmuResult::StateChangeFlag(cx, cy) = emu_result {
      cx == 0xc8 && cy == 9 && s.y_pos >= 0x1a500
    } else { false }
  }
}
