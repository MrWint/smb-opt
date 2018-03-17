use blockbuffer::world2::*;
#[allow(unused_imports)] use emu::{Emu, EmuResult};
#[allow(unused_imports)] use emu::inputs::*;
use heuristics::SearchGoal;
use heuristics::xpos::XPosHeuristic;
use options::*;
use state::{CompressedState, Dir, Dist, PlayerState, State};


/// Floor clip using Cheep Cheep in 7-3 as big Mario
#[allow(dead_code)]
pub struct W73FloorClip {
  max_x_pos: i32,
  h: XPosHeuristic,
}
impl Options for W73FloorClip {
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
impl super::SmbSearchCase for W73FloorClip {
  type SearchGoal = Self;
  type StateStore = ::store::VecHashMap<CompressedState<Self, [u8; 10]>, Dist>;

  type BlockBuffer = BB23;

  fn start_states() -> Vec<State> {
    let s = State {
      x_pos: 0xcdbc0,
      y_pos: 0x1c050,
      x_spd: 0x23ec,
      y_spd: -0x300, // 0xfd00,
      player_state: PlayerState::FALLING,
      moving_dir: Dir::RIGHT,
      facing_dir: Dir::LEFT,
      v_force: <Self as Options>::Platform::V_FORCE_FALL_RUNNING,
      v_force_down: <Self as Options>::Platform::V_FORCE_FALL_RUNNING,
      x_spd_abs: 0x21,
      running_speed: false,
      collision_bits: Dir::LR,
      is_crouching: false,
      jump_swim_timer: 0,
      running_timer : 1,
      left_screen_edge_pos: 0x6b,
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
  const INITIAL_SEARCH_DISTANCE: Dist = 50;
  const SEARCH_SPACE_SIZE_HINT: usize = 800000000;
}
impl SearchGoal for W73FloorClip {
  fn new() -> Self { return Self { max_x_pos: 0, h: XPosHeuristic::new::<Self>(&<Self as super::SmbSearchCase>::start_states()) }; }
  fn distance_to_goal_heuristic(&self, s: &mut State, _steps_already_taken: Dist) -> Option<Dist> {
    if s.y_pos < 0x1b500 { return None; } // no floor clip

    // Some(self.h.get_steps_until_x_pos_at_least(s, 0xd21c0 - 0x20))
    Some(self.h.get_steps_until_x_pos_at_least(s, self.max_x_pos + 0x10))
  }
  fn is_goal_state(&self, _s: &State, _: &EmuResult) -> bool {
    _s.x_pos >= 0xd2120 && _s.y_pos >= 0x1c000
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



/// Full flag pole glitch in 7-3 from floor clip
#[allow(dead_code)]
pub struct W73FloorFlag {
  h: XPosHeuristic,
}
impl Options for W73FloorFlag {
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
impl super::SmbSearchCase for W73FloorFlag {
  type SearchGoal = Self;
  type StateStore = ::store::VecHashMap<CompressedState<Self, [u8; 10]>, Dist>;

  type BlockBuffer = BB23;

  fn start_states() -> Vec<State> {
    let s = State {
      x_pos: 0xdee20,
      y_pos: 0x1c070,
      x_spd: 0x2814,
      y_spd: 0x0,
      player_state: PlayerState::STANDING,
      moving_dir: Dir::RIGHT,
      facing_dir: Dir::RIGHT,
      v_force: <Self as Options>::Platform::V_FORCE_FALL_RUNNING,
      v_force_down: <Self as Options>::Platform::V_FORCE_FALL_RUNNING,
      x_spd_abs: 0x21,
      running_speed: true,
      collision_bits: Dir::RIGHT,
      is_crouching: false,
      jump_swim_timer: 0,
      running_timer : 10,
      left_screen_edge_pos: 0x72,
      side_collision_timer: 0,
      collected_coins: 0,
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
impl SearchGoal for W73FloorFlag {
  fn new() -> Self { return Self { h: XPosHeuristic::new::<Self>(&<Self as super::SmbSearchCase>::start_states()) }; }
  fn distance_to_goal_heuristic(&self, s: &mut State, _steps_already_taken: Dist) -> Option<Dist> {
    Some(self.h.get_steps_until_x_pos_at_least(s, 0xe0600))
  }
  fn is_goal_state(&self, s: &State, emu_result: &EmuResult) -> bool {
    if let &EmuResult::StateChangeFlag(cx, cy) = emu_result {
      cx == 0xe1 && cy == 9 && s.y_pos >= 0x1a500
    } else { false }
  }
}
