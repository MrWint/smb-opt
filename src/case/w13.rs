use blockbuffer::world1::*;
use emu::{Emu, EmuResult, SmbEmu};
#[allow(unused_imports)] use emu::inputs::*;
use heuristics::{BoundsHeuristic, SearchGoal};
use heuristics::xpos::XPosHeuristic;
use options::*;
use state::{CompressedState, Dir, Dist, PlayerState, State};

fn w12_start<O: Options>() -> State {
  SmbEmu::<O, BB12>::iterate_entrance(State {
    x_pos: 0x2800,
    y_pos: 0x12000,
    x_spd: 0x0,
    v_force: O::Platform::V_FORCE_AREA_INIT,
    y_spd: 0x0,
    v_force_down: O::Platform::V_FORCE_AREA_INIT,
    facing_dir: Dir::RIGHT,
    moving_dir: Dir::empty(),
    player_state: PlayerState::FALLING,
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
  })
}

/// Initial speed-up starting in 1-2
/// Input sequence: [34x R, 1x L|R, 1x B|R, 2x L|R, 1x B|R, 1x L, 1x A|R, 20x R, 10x B|R] (len: 71)
#[allow(dead_code)]
pub struct W12Speedup {
  h: BoundsHeuristic,
}
impl SearchGoal for W12Speedup {
  fn new() -> Self { return Self { h: BoundsHeuristic::new::<<Self as super::SmbSearchCase>::Options>(&<Self as super::SmbSearchCase>::start_states()) }; }
  fn distance_to_goal_heuristic(&self, s: &mut State, _: Dist) -> Option<Dist> {
    Some(self.h.get_steps_until_x_pos_at_least(s, 0x9190))
  }
  fn is_goal_state(&self, s: &State, _: &EmuResult) -> bool {
    s.x_pos >= 0x9190
  }
}
impl super::SmbSearchCase for W12Speedup {
  type SearchGoal = Self;
  type StateStore = ::store::VecHashMap<CompressedState<Self::Options>, Dist>;

  type BlockBuffer = BB12;

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
    vec![w12_start::<Self::Options>()]
  }
  const SEARCH_SPACE_SIZE_HINT: usize = 59;
}

/// Collect mushroom in 1-3 as small Mario
/// Input sequence: [1x A, 17x R, 1x L, 1x NIL, 1x L, 5x R, 1x A|R, 3x R, 2x NIL, 1x L, 9x R, 1x B|R, 1x A|L, 1x A|R, 1x A|L, 1x A, 2x R, 1x B|R, 16x R] (len: 66)
#[allow(dead_code)]
pub struct W13PowerupSmall {
  h: BoundsHeuristic,
}
impl SearchGoal for W13PowerupSmall {
  fn new() -> Self { return Self { h: BoundsHeuristic::new::<<Self as super::SmbSearchCase>::Options>(&<Self as super::SmbSearchCase>::start_states()) }; }
  fn distance_to_goal_heuristic(&self, s: &mut State, steps_already_taken: Dist) -> Option<Dist> {
    if steps_already_taken == 17 && s.y_pos >= 0x19100 && s.x_pos < 0x39e00 { return None; } // collided with moving platform
    if steps_already_taken == 19 && s.y_pos >= 0x19400 && s.x_pos < 0x39e00 { return None; } // collided with moving platform
    if steps_already_taken == 21 && s.y_pos >= 0x19700 && s.x_pos < 0x39e00 { return None; } // collided with moving platform

    const MAX_STEPS_TO_HIT: Dist = 29 + 4;
    if (steps_already_taken < MAX_STEPS_TO_HIT-3 && s.powerup_block_hit) || (steps_already_taken >= MAX_STEPS_TO_HIT && !s.powerup_block_hit) {
      return None; // not hit powerup block in valid timeframe
    }

    for i in 0..12 {
      if !s.powerup_collected
          && (steps_already_taken == MAX_STEPS_TO_HIT + 16 + 4*i || steps_already_taken == MAX_STEPS_TO_HIT + 18 + 4*i)
          && s.y_pos < 0x19500 - 0x100*i as i32
          && s.y_pos >= 0x17c00 - 0x100*i as i32
          && s.x_pos >= 0x3a500
          && s.x_pos < 0x3bc00 {
        s.powerup_collected = true;
        s.player_state = PlayerState::STANDING;
        break;
      }
    }

    if s.x_pos >= 0x3c000 && !s.powerup_collected { return None } // missed powerup collection
    Some(self.h.get_steps_until_x_pos_at_least(s, 0x3de80))
  }
  fn is_goal_state(&self, s: &State, _: &EmuResult) -> bool {
    s.x_pos >= 0x3de80
  }
}
impl super::SmbSearchCase for W13PowerupSmall {
  type SearchGoal = Self;
  type StateStore = ::store::VecHashMap<CompressedState<Self::Options>, Dist>;

  type BlockBuffer = BB13;

  type CoinHandler = IgnoreCoins;
  type Platform = NTSC;
  type PlayerSize = Small;
  type PlayerSwimming = NotSwimming;
  type PowerupHandler = SinglePowerupHandler<::typenum::U59, ::typenum::U8>; // (0x3b, 0x8)
  type RunningTimer = NoRunningTimer;
  type ScrollPos = NoScrollPos;
  type VerticalPipeHandler = IgnoreVerticalPipes;
  type YPosFractionalBehavior = KeepYPosFractionals;

  fn start_states() -> Vec<State> {
    // super::with_smaller_x_pos::<Self::Options>(super::with_left_and_right_facing_dir(super::with_all_x_spd_subpixels(
    vec![State {
      x_pos: 0x37590,
      y_pos: 0x17e50,
      x_spd: 0x2800,
      v_force: Self::Platform::V_FORCE_JUMP_RUNNING,
      y_spd: -0x320, // 0xfce0
      v_force_down: Self::Platform::V_FORCE_FALL_RUNNING,
      facing_dir: Dir::LEFT,
      moving_dir: Dir::RIGHT,
      player_state: PlayerState::JUMPING,
      x_spd_abs: 0x21,
      running_speed: true,
      collision_bits: Dir::LR,
      side_collision_timer: 0,
      left_screen_edge_pos: 0x5,
      jump_swim_timer: 20,
      running_timer : 0,
      is_crouching: false,
      coin_collected: false,
      powerup_block_hit: false,
      powerup_collected: false,
    }]
    // )), 16)
  }
  const INITIAL_SEARCH_DISTANCE: Dist = 66;
  const SEARCH_SPACE_SIZE_HINT: usize = 79761384;
}

/// Collect mushroom in 1-3 as big Mario
/// Input sequence: [1x A, 17x R, 2x NIL, 6x R, 1x NIL, 1x A|R, 4x R, 2x NIL, 1x R, 2x L|R, 1x B|R, 1x L|R, 1x B|R, 1x L, 1x A|R, 1x L, 2x R, 1x A|R, 20x R] (len: 66)
#[allow(dead_code)]
pub struct W13PowerupBig {
  max_x_pos: i32,
  h: XPosHeuristic,
}
impl SearchGoal for W13PowerupBig {
  fn new() -> Self { return Self { max_x_pos: 0, h: XPosHeuristic::new::<<Self as super::SmbSearchCase>::Options>(&<Self as super::SmbSearchCase>::start_states()) }; }
  fn distance_to_goal_heuristic(&self, s: &mut State, steps_already_taken: Dist) -> Option<Dist> {
    if steps_already_taken == 17 && s.y_pos >= 0x19100 && s.x_pos < 0x39f00 { return None; } // collided with moving platform
    if steps_already_taken == 19 && s.y_pos >= 0x19400 && s.x_pos < 0x39f00 { return None; } // collided with moving platform
    if steps_already_taken == 21 && s.y_pos >= 0x19700 && s.x_pos < 0x39f00 { return None; } // collided with moving platform

    const MAX_STEPS_TO_HIT: Dist = 29;// + 4;
    if (steps_already_taken < MAX_STEPS_TO_HIT-3 && s.powerup_block_hit) || (steps_already_taken >= MAX_STEPS_TO_HIT && !s.powerup_block_hit) {
      return None; // not hit powerup block in valid timeframe
    }

    for i in 0..12 {
      if !s.powerup_collected
          && (steps_already_taken == MAX_STEPS_TO_HIT + 16 + 4*i || steps_already_taken == MAX_STEPS_TO_HIT + 18 + 4*i)
          && s.y_pos < 0x19500 + if s.is_crouching { 0 } else { 0xc00 } - 0x100*i as i32
          && s.y_pos >= 0x17c00 - 0x100*i as i32
          && s.x_pos >= 0x3a400
          && s.x_pos < 0x3bd00 {
        s.powerup_collected = true;
        s.player_state = PlayerState::STANDING;
        break;
      }
    }

    if s.x_pos >= 0x3c000 && !s.powerup_collected { return None } // missed powerup collection
    Some(self.h.get_steps_until_x_pos_at_least(s, 0x3ef60))
    // Some(self.h.get_steps_until_x_pos_at_least(s, self.max_x_pos + 0x10))
  }
  fn is_goal_state(&self, s: &State, _: &EmuResult) -> bool {
    // false
    s.x_pos >= 0x3ef60
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
impl super::SmbSearchCase for W13PowerupBig {
  type SearchGoal = Self;
  type StateStore = ::store::VecHashMap<CompressedState<Self::Options>, Dist>;

  type BlockBuffer = BB13;

  type CoinHandler = IgnoreCoins;
  type Platform = NTSC;
  type PlayerSize = Big;
  type PlayerSwimming = NotSwimming;
  type PowerupHandler = SinglePowerupHandler<::typenum::U59, ::typenum::U8>; // (0x3b, 0x8)
  type RunningTimer = NoRunningTimer;
  type ScrollPos = NoScrollPos;
  type VerticalPipeHandler = IgnoreVerticalPipes;
  type YPosFractionalBehavior = KeepYPosFractionals;

  fn start_states() -> Vec<State> {
    // super::with_smaller_x_pos::<Self::Options>(super::with_left_and_right_facing_dir(super::with_all_x_spd_subpixels(
    vec![State {
      x_pos: 0x37590,
      y_pos: 0x17e50,
      x_spd: 0x2800,
      v_force: Self::Platform::V_FORCE_JUMP_RUNNING,
      y_spd: -0x320, // 0xfce0
      v_force_down: Self::Platform::V_FORCE_FALL_RUNNING,
      facing_dir: Dir::LEFT,
      moving_dir: Dir::RIGHT,
      player_state: PlayerState::JUMPING,
      x_spd_abs: 0x21,
      running_speed: true,
      collision_bits: Dir::LR,
      side_collision_timer: 0,
      left_screen_edge_pos: 0x5,
      jump_swim_timer: 20,
      running_timer : 0,
      is_crouching: true,
      coin_collected: false,
      powerup_block_hit: false,
      powerup_collected: false,
    }]
    // )), 16)
  }
  const INITIAL_SEARCH_DISTANCE: Dist = 66;
  const SEARCH_SPACE_SIZE_HINT: usize = 37516298;
}
