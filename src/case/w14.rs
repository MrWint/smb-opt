use blockbuffer::world1::*;
#[allow(unused_imports)] use emu::{Emu, EmuResult};
#[allow(unused_imports)] use emu::inputs::*;
use heuristics::SearchGoal;
use heuristics::xpos::XPosHeuristic;
use options::*;
use state::{CompressedState, Dir, Dist, PlayerState, State};

fn w14_start<O: Options>() -> State {
  State {
    x_pos: 0x2800,
    y_pos: 0x15000,
    x_spd: 0x0,
    y_spd: 0x0,
    player_state: PlayerState::STANDING,
    moving_dir: Dir::empty(),
    facing_dir: Dir::RIGHT,
    v_force: O::Platform::V_FORCE_AREA_INIT,
    v_force_down: O::Platform::V_FORCE_AREA_INIT,
    x_spd_abs: 0x0,
    running_speed: false,
    collision_bits: Dir::LR,
    is_crouching: false,
    jump_swim_timer: 0,
    running_timer : 0,
    left_screen_edge_pos: 0x0,
    side_collision_timer: 0,
    coin_collected: false,
    powerup_block_hit: false,
    powerup_collected: false,
    parity: 0,
  }
}

/// Initial speed-up starting in 1-4
/// Input sequence: [1x L|R, 1x A|R, 5x R, 1x A|R, 9x R, 1x B|R, 1x L|R, 1x B|R, 1x L|R, 1x B|R, 1x L, 1x A|R, 20x R, 15x B|R] (len: 59)
#[allow(dead_code)]
pub struct W14Speedup {
  h: XPosHeuristic,
}
impl Options for W14Speedup {
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
impl super::SmbSearchCase for W14Speedup {
  type SearchGoal = Self;
  type StateStore = ::store::VecHashMap<CompressedState<Self, [u8; 10]>, Dist>;

  type BlockBuffer = BB14;

  fn start_states() -> Vec<State> {
    vec![w14_start::<Self>()]
  }
  const SEARCH_SPACE_SIZE_HINT: usize = 0;
}
impl SearchGoal for W14Speedup {
  fn new() -> Self { return Self { h: XPosHeuristic::new::<Self>(&<Self as super::SmbSearchCase>::start_states()) }; }
  fn distance_to_goal_heuristic(&self, s: &mut State, _: Dist) -> Option<Dist> {
    Some(self.h.get_steps_until_x_pos_at_least(s, 0x92a0))
  }
  fn is_goal_state(&self, s: &State, _: &EmuResult) -> bool {
    s.x_pos >= 0x92a0
  }
}


/// Collect fire flower in 1-4 as big Mario
/// Input sequence: [3x NIL, 2x A|L, 1x A, 3x L, 4x NIL, 7x R, 7x L|R, 1x B|R, 4x A, 1x R, 1x NIL, 2x R, 1x A|R, 25x R] (len: 62)
#[allow(dead_code)]
pub struct W14Powerup {
  max_x_pos: i32,
  h: XPosHeuristic,
}
impl Options for W14Powerup {
  type CoinHandler = IgnoreCoins;
  type Platform = NTSC;
  type PlayerSize = Big;
  type Swim = NotSwimming;
  type PowerupHandler = SinglePowerupHandler<::typenum::U30, ::typenum::U4>; // (0x1e, 0x4)
  type RunningTimer = NoRunningTimer;
  type ScrollPos = NoScrollPos;
  type Parity = Parity8;
  type VerticalPipeHandler = IgnoreVerticalPipes;
  type YPosFractionalBehavior = KeepYPosFractionals;
}
impl super::SmbSearchCase for W14Powerup {
  type SearchGoal = Self;
  type StateStore = ::store::VecHashMap<CompressedState<Self, [u8; 11]>, Dist>;

  type BlockBuffer = BB14;

  fn start_states() -> Vec<State> {
    let s = State {
      x_pos: 0x1c620,
      y_pos: 0x18028,
      x_spd: 0x2818,
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
      jump_swim_timer: 11,
      running_timer : 0,
      left_screen_edge_pos: 0x56,
      side_collision_timer: 0,
      coin_collected: false,
      powerup_block_hit: false,
      powerup_collected: false,
      parity: 0,
    };
    println!("start state {}", s);
    // super::with_smaller_x_pos::<Self>(super::with_left_and_right_facing_dir(super::with_all_x_spd_subpixels(
    vec![s]
    // )), 0)
  }
  const INITIAL_SEARCH_DISTANCE: Dist = 62;
  const SEARCH_SPACE_SIZE_HINT: usize = 12550052;
}
impl SearchGoal for W14Powerup {
  fn new() -> Self { return Self { max_x_pos: 0x22500, h: XPosHeuristic::new::<Self>(&<Self as super::SmbSearchCase>::start_states()) }; }
  fn distance_to_goal_heuristic(&self, s: &mut State, steps_already_taken: Dist) -> Option<Dist> {
    if s.y_pos >= 0x18500 { return None } // too low

    if s.powerup_block_hit && !s.powerup_collected && s.y_spd < 0 && s.x_spd < 0x1900 { return None } // no running jump to collect powerup
    if s.powerup_block_hit && !s.powerup_collected && s.x_pos >= 0x1e210 && s.player_state == PlayerState::STANDING { return None } // jumped too late, will miss fireflower in running jump

    const MAX_STEPS_TO_HIT: Dist = 10;
    if (steps_already_taken < MAX_STEPS_TO_HIT-3 && s.powerup_block_hit) || (steps_already_taken >= MAX_STEPS_TO_HIT && !s.powerup_block_hit) {
      return None; // not hit powerup block in valid timeframe
    }

    for i in 0..12 {
      if !s.powerup_collected
          && (steps_already_taken == MAX_STEPS_TO_HIT + 16 + 4*i || steps_already_taken == MAX_STEPS_TO_HIT + 18 + 4*i)
          && s.y_pos < 0x15500 + if s.is_crouching { 0 } else { 0xc00 } - 0x100*i as i32
          && s.y_pos >= 0x13c00 - 0x100*i as i32
          && s.x_pos >= 0x1d400
          && s.x_pos < 0x1ed00 {
        if s.x_spd < 0x1900 { return None; } // too slow
        s.powerup_collected = true;
        s.player_state = PlayerState::STANDING;
        break;
      }
    }

    if !s.powerup_collected && steps_already_taken > MAX_STEPS_TO_HIT + 18 + 4*11 { return None } // missed powerup collection
    if !s.powerup_collected && s.x_pos >= 0x1ed00 { return None } // missed powerup collection
    Some(self.h.get_steps_until_x_pos_at_least(s, 0x228a0))
    // Some(self.h.get_steps_until_x_pos_at_least(s, self.max_x_pos + 0x10))
  }
  fn is_goal_state(&self, s: &State, _: &EmuResult) -> bool {
    s.powerup_collected && s.x_pos >= 0x228a0
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
