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
    collected_coins: 0,
    powerup_block_hit: false,
    powerup_collected: false,
    parity: 0,
  })
}

/// Initial speed-up starting in 1-2
/// Input sequence: [34x R, 1x L|R, 1x B|R, 2x L|R, 1x B|R, 1x L, 1x A|R, 20x R, 10x B|R] (len: 71)
#[allow(dead_code)]
pub struct W12Speedup {
  h: XPosHeuristic,
}
impl Options for W12Speedup {
  type CoinHandler = IgnoreCoins;
  type Platform = NTSC;
  type PlayerSize = Small;
  type Swim = NotSwimming;
  type PowerupHandler = NoPowerups;
  type RunningTimer = NoRunningTimer;
  type ScrollPos = NoScrollPos;
  type Parity = NoParity;
  type VerticalPipeHandler = IgnoreVerticalPipes;
  type YPosFractionalBehavior = KeepYPosFractionals;
}
impl super::SmbSearchCase for W12Speedup {
  type SearchGoal = Self;
  type StateStore = ::store::VecHashMap<CompressedState<Self, [u8; 10]>, Dist>;

  type BlockBuffer = BB12;

  fn start_states() -> Vec<State> {
    vec![w12_start::<Self>()]
  }
  const SEARCH_SPACE_SIZE_HINT: usize = 59;
}
impl SearchGoal for W12Speedup {
  fn new() -> Self { return Self { h: XPosHeuristic::new::<Self>(&<Self as super::SmbSearchCase>::start_states()) }; }
  fn distance_to_goal_heuristic(&self, s: &mut State, _: Dist) -> Option<Dist> {
    Some(self.h.get_steps_until_x_pos_at_least(s, 0x9190))
  }
  fn is_goal_state(&self, s: &State, _: &EmuResult) -> bool {
    s.x_pos >= 0x9190
  }
}

/// Collect mushroom in 1-2
/// Input sequence: [3x NIL, 1x A|R, 1x A, 2x A|R, 2x A, 2x A|R, 2x R, 14x NIL, 4x A|L, 6x A|R, 1x A, 1x A|R, 6x R, 8x L|R, 1x B|R, 1x L, 1x A|R, 12x R] (len: 68)
#[allow(dead_code)]
pub struct W12Powerup {
  h: XPosHeuristic,
}
impl Options for W12Powerup {
  type CoinHandler = IgnoreCoins;
  type Platform = NTSC;
  type PlayerSize = Small;
  type Swim = NotSwimming;
  type PowerupHandler = SinglePowerupHandler<::typenum::U10, ::typenum::U7>; // (0xa, 0x7)
  type RunningTimer = NoRunningTimer;
  type ScrollPos = NoScrollPos;
  type Parity = NoParity;
  type VerticalPipeHandler = IgnoreVerticalPipes;
  type YPosFractionalBehavior = KeepYPosFractionals;
}
impl super::SmbSearchCase for W12Powerup {
  type SearchGoal = Self;
  type StateStore = ::store::VecHashMap<CompressedState<Self, [u8; 10]>, Dist>;

  type BlockBuffer = BB12;

  fn start_states() -> Vec<State> {
    vec![State {
      x_pos: 0x7890,
      y_pos: 0x1b028,
      x_spd: 0x2888,
      v_force: <Self as Options>::Platform::V_FORCE_FALL_RUNNING,
      y_spd: 0x0,
      v_force_down: <Self as Options>::Platform::V_FORCE_FALL_RUNNING,
      facing_dir: Dir::LEFT,
      moving_dir: Dir::RIGHT,
      player_state: PlayerState::STANDING,
      x_spd_abs: 0x21,
      running_speed: false,
      collision_bits: Dir::LR,
      side_collision_timer: 0,
      left_screen_edge_pos: 0x18,
      jump_swim_timer: 11,
      running_timer : 0,
      is_crouching: false,
      collected_coins: 0,
      powerup_block_hit: false,
      powerup_collected: false,
      parity: 0,
    }]
  }
  const INITIAL_SEARCH_DISTANCE: Dist = 68;
  const SEARCH_SPACE_SIZE_HINT: usize = 277409196; // INITIAL_SEARCH_DISTANCE=0: 277624442;
}
impl SearchGoal for W12Powerup {
  fn new() -> Self { return Self { h: XPosHeuristic::new::<Self>(&<Self as super::SmbSearchCase>::start_states()) }; }
  fn distance_to_goal_heuristic(&self, s: &mut State, steps_already_taken: Dist) -> Option<Dist> {
    const MAX_STEPS_TO_HIT: Dist = 14;
    if (steps_already_taken < MAX_STEPS_TO_HIT-3 && s.powerup_block_hit) || (steps_already_taken >= MAX_STEPS_TO_HIT && !s.powerup_block_hit) {
      return None; // not hit powerup block in valid timeframe
    }

    for i in 0..12 {
      if !s.powerup_collected
          && (steps_already_taken == MAX_STEPS_TO_HIT + 16 + 4*i || steps_already_taken == MAX_STEPS_TO_HIT + 18 + 4*i)
          && s.y_pos < 0x18500 - 0x100*i as i32
          && s.y_pos >= 0x16c00 - 0x100*i as i32
          && s.x_pos >= 0x9500
          && s.x_pos < 0xac00 {
        s.powerup_collected = true;
        s.player_state = PlayerState::STANDING;
        break;
      }
    }

    if s.x_pos >= 0xa000 && !s.powerup_collected { return None } // missed powerup collection
    Some(self.h.get_steps_until_x_pos_at_least(s, 0xbe60))
  }
  fn is_goal_state(&self, s: &State, _: &EmuResult) -> bool {
    s.x_pos >= 0xbe60
  }
}

/// Exit pipe to half flag pole glitch
/// Input sequence: [1x L|R, 1x A|R, 19x R, 1x NIL, 1x B|R, 1x L, 4x R, 2x A|R, 11x R, 4x B|R, 1x A, 9x R, 3x B|R, 1x A, 9x R, 14x B|R, 1x A, 36x R, 1x L, 1x NIL, 4x L, 7x R, 1x L, 1x NIL, 1x R, 1x A|L, 1x NIL, 1x R] (len: 138)
/// Input sequence: [1x L|R, 1x A|R, 19x R, 1x NIL, 1x B|R, 1x B|L, 4x B|R, 2x A|B|R, 2x B|R, 9x B, 2x B|R, 1x R, 1x B|L, 1x A, 9x R, 3x B|R, 1x A, 9x R, 14x B|R, 1x A, 34x R, 4x NIL, 2x L, 1x NIL, 2x L, 2x NIL, 4x R, 1x L, 1x NIL, 1x R, 1x A|L, 1x NIL, 1x R] (len: 138)
#[allow(dead_code)]
pub struct W12Flag {
  heuristic: BoundsHeuristic,
}
impl Options for W12Flag {
  type CoinHandler = IgnoreCoins;
  type Platform = NTSC;
  type PlayerSize = Small;
  type Swim = NotSwimming;
  type PowerupHandler = NoPowerups;
  type RunningTimer = NoRunningTimer;
  type ScrollPos = NoScrollPos;
  type Parity = NoParity;
  type VerticalPipeHandler = IgnoreVerticalPipes;
  type YPosFractionalBehavior = KeepYPosFractionals;
}
impl super::SmbSearchCase for W12Flag {
  type SearchGoal = Self;
  type StateStore = ::store::VecHashMap<CompressedState<Self, [u8; 10]>, Dist>;

  type BlockBuffer = BB11;

  fn start_states() -> Vec<State> {
    let s = State {
      x_pos: 0xb3800,
      y_pos: 0x19000,
      x_spd: 0x0,
      v_force: <Self as Options>::Platform::V_FORCE_AREA_INIT,
      y_spd: 0x0,
      v_force_down: <Self as Options>::Platform::V_FORCE_AREA_INIT,
      facing_dir: Dir::RIGHT,
      moving_dir: Dir::empty(),
      player_state: PlayerState::STANDING,
      x_spd_abs: 0x0,
      running_speed: false,
      collision_bits: Dir::LR,
      side_collision_timer: 0,
      left_screen_edge_pos: 0x0,
      jump_swim_timer: 0,
      running_timer : 0,
      is_crouching: false,
      collected_coins: 0,
      powerup_block_hit: false,
      powerup_collected: false,
      parity: 0,
    };
    let s = Self::Emu::run_steps_nr(s, &[L|R, A|R]);
    let s = Self::Emu::run_steps_nr(s, &[R; 19]);
    let s = Self::Emu::run_steps_nr(s, &[NIL, B|R, B|L]);
    let s = Self::Emu::run_steps_nr(s, &[B|R; 4]);
    let s = Self::Emu::run_steps_nr(s, &[A|B|R; 2]);
    let s = Self::Emu::run_steps_nr(s, &[B|R; 2]);
    let s = Self::Emu::run_steps_nr(s, &[B; 9]);
    let s = Self::Emu::run_steps_nr(s, &[B|R; 2]);
    let s = Self::Emu::run_steps_nr(s, &[B|R, B|L]);
    vec![s]
  }
  //const INITIAL_SEARCH_DISTANCE: Dist = 138;
  const SEARCH_SPACE_SIZE_HINT: usize = 30383304;
}
impl SearchGoal for W12Flag {
  fn new() -> Self { Self { heuristic: BoundsHeuristic::new::<Self>(&<Self as super::SmbSearchCase>::start_states()) } }
  fn distance_to_goal_heuristic(&self, s: &mut State, _: Dist) -> Option<Dist> {
    if s.x_pos < 0xc5310 - 0x5000 { Some(7 + 1 + 32 + self.heuristic.get_steps_until_x_pos_at_least(s, 0xc5310 - 0x5000)) } // ensure fast speedup
    else if s.x_pos < 0xc53f0 { Some(7 + self.heuristic.get_steps_until_bounds_at_least(s, 0xc53f0, 0x19b00)) } // align for clip
    else { Some(self.heuristic.get_steps_until_bounds_at_least(s, 0xc5600, 0x1a200)) } // do the clip
  }
  fn is_goal_state(&self, s: &State, emu_result: &EmuResult) -> bool {
    if let &EmuResult::StateChangeFlag(cx, cy) = emu_result {
      cx == 0xc6 && cy == 9 && s.y_pos >= 0x1a200 && (s.x_pos & 0xf0) >= 0x70
    } else { false }
  }
}
